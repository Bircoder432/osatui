use crate::{
    api::ApiClient,
    app::{App, AppMode},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_event(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    app.state_mut().clear_error_if_expired();

    match app.state().mode {
        AppMode::Normal => handle_normal_mode(app, key).await,
        AppMode::Selector => handle_selector_mode(app, key).await,
        AppMode::Setup => handle_setup_mode(app, key).await,
    }
}

async fn handle_normal_mode(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    let keymap = app.state().config.keymap().clone();

    match key.code {
        code if code == keymap.exit() => {
            app.quit();
        }
        code if code == keymap.selector() => {
            let mut api = app.take_api();
            if let Some(ref mut api) = api {
                app.state_mut()
                    .enter_selector(crate::ui::screens::selector::SelectionStage::College);
                app.state_mut().load_colleges(api).await?;
            }
            app.set_api(api.unwrap());
        }
        code if code == keymap.settings() => {
            app.state_mut().enter_setup();
        }
        code if code == keymap.prev_day() => {
            app.state_mut().prev_day();
            let date = app.state().current_date;
            let schedules = if let Some(api) = app.api() {
                api.fetch(&date).await?
            } else {
                Vec::new()
            };
            app.state_mut().schedules = schedules;
        }
        code if code == keymap.cur_day() => {
            app.state_mut().go_today();
            let date = app.state().current_date;
            let schedules = if let Some(api) = app.api() {
                api.fetch(&date).await?
            } else {
                Vec::new()
            };
            app.state_mut().schedules = schedules;
        }
        code if code == keymap.next_day() => {
            app.state_mut().next_day();
            let date = app.state().current_date;
            let schedules = if let Some(api) = app.api() {
                api.fetch(&date).await?
            } else {
                Vec::new()
            };
            app.state_mut().schedules = schedules;
        }
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            let date = app.state().current_date; // <-- ИСПРАВЛЕНО: получаем date ДО api_mut()
            if let Some(api) = app.api_mut() {
                api.clear_cache().await?;
                let schedules = api.fetch(&date).await?;
                app.state_mut().schedules = schedules;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn handle_selector_mode(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Enter => {
            handle_selector_selection(app).await?;
        }
        KeyCode::Down => app.state_mut().next_item(),
        KeyCode::Up => app.state_mut().prev_item(),
        KeyCode::Right => app.state_mut().next_page(),
        KeyCode::Left => app.state_mut().prev_page(),
        KeyCode::Esc => {
            app.state_mut().enter_normal();
        }
        _ => {}
    }

    Ok(())
}

async fn handle_selector_selection(app: &mut App) -> anyhow::Result<()> {
    use crate::ui::screens::selector::SelectionStage;

    let stage = app.state().selection_stage;

    match stage {
        SelectionStage::College => {
            if let Some(college) = app.state().get_selected_college().cloned() {
                let college_id = college.college_id;
                app.state_mut().selected_college = Some(college);
                app.state_mut().selection_stage = SelectionStage::Campus;

                let mut api = app.take_api();
                if let Some(ref mut api) = api {
                    app.state_mut().load_campuses(api, college_id).await?;
                }
                app.set_api(api.unwrap());
            }
        }
        SelectionStage::Campus => {
            if let Some(campus) = app.state().get_selected_campus().cloned() {
                let campus_id = campus.id;
                app.state_mut().selected_campus = Some(campus);
                app.state_mut().selection_stage = SelectionStage::Group;

                let mut api = app.take_api();
                if let Some(ref mut api) = api {
                    app.state_mut().load_groups(api, campus_id).await?;
                }
                app.set_api(api.unwrap());
            }
        }
        SelectionStage::Group => {
            let group = app.state().get_selected_group().cloned();
            let college = app.state().selected_college.clone();
            let campus = app.state().selected_campus.clone();

            if let Some(group) = group {
                if let Some(ref college) = college {
                    app.state_mut()
                        .config
                        .set_college(college.college_id, Some(college.name.clone()));
                }
                if let Some(ref campus) = campus {
                    app.state_mut()
                        .config
                        .set_campus(campus.id, Some(campus.name.clone()));
                }
                app.state_mut()
                    .config
                    .set_group(group.id, Some(group.name.clone()));
                app.state_mut().config.save().await?;

                let new_api = ApiClient::new(app.state().config.clone()).await?;
                app.set_api(new_api);

                let mut schedules = Vec::new();
                if let Some(api) = app.api() {
                    schedules = api.fetch(&app.state().current_date).await?;
                }
                app.state_mut().schedules = schedules;
                app.state_mut().enter_normal();
            }
        }
    }

    Ok(())
}

async fn handle_setup_mode(app: &mut App, key: KeyEvent) -> anyhow::Result<()> {
    use crate::app::state::SetupField;

    match key.code {
        KeyCode::Enter => {
            let state = app.state();
            let url = state.setup_api_url.trim().to_string();
            let college_id = state.setup_college_id.parse().unwrap_or(1);
            let campus_id = state.setup_campus_id.parse().unwrap_or(1);
            let group_name = state.setup_group_name.trim().to_string();

            app.state_mut().config.set_api_url(url);
            app.state_mut().config.set_college(college_id, None);
            app.state_mut().config.set_campus(campus_id, None);
            app.state_mut().config.set_group(0, Some(group_name));

            app.state_mut().config.save().await?;
            app.reload_api().await?;
            app.state_mut().enter_normal();
        }
        KeyCode::Tab => {
            app.state_mut().setup_field = match app.state().setup_field {
                SetupField::ApiUrl => SetupField::CollegeId,
                SetupField::CollegeId => SetupField::CampusId,
                SetupField::CampusId => SetupField::GroupName,
                SetupField::GroupName => SetupField::ApiUrl,
            };
        }
        KeyCode::BackTab => {
            app.state_mut().setup_field = match app.state().setup_field {
                SetupField::ApiUrl => SetupField::GroupName,
                SetupField::CollegeId => SetupField::ApiUrl,
                SetupField::CampusId => SetupField::CollegeId,
                SetupField::GroupName => SetupField::CampusId,
            };
        }
        KeyCode::Char(c) => match app.state().setup_field {
            SetupField::ApiUrl => app.state_mut().setup_api_url.push(c),
            SetupField::CollegeId => app.state_mut().setup_college_id.push(c),
            SetupField::CampusId => app.state_mut().setup_campus_id.push(c),
            SetupField::GroupName => app.state_mut().setup_group_name.push(c),
        },
        KeyCode::Backspace => match app.state().setup_field {
            SetupField::ApiUrl => {
                app.state_mut().setup_api_url.pop();
            }
            SetupField::CollegeId => {
                app.state_mut().setup_college_id.pop();
            }
            SetupField::CampusId => {
                app.state_mut().setup_campus_id.pop();
            }
            SetupField::GroupName => {
                app.state_mut().setup_group_name.pop();
            }
        },
        KeyCode::Esc => {
            app.state_mut().enter_normal();
        }
        _ => {}
    }

    Ok(())
}

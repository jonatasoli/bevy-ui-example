use bevy::prelude::*;

#[derive(Resource)]
enum AppState {
    MainMenu,
    NewScreen,
    Credits,
}

#[derive(Component, Debug)]
enum ButtonTag {
    Start,
    Credits,
    Exit,
    Main,
}

#[derive(Component)]
struct MenuUi;

mod ui_components {
    use bevy::prelude::*;

    pub const BACKGROUND_COLOR: Color = Color::srgb(0.10, 0.08, 0.08);

    pub fn default_button_style() -> Style {
        Style {
            width: Val::Px(150.0),
            height: Val::Px(150.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        }
    }

    pub fn default_text_style(font: Handle<Font>) -> TextStyle {
        TextStyle {
            font,
            font_size: 40.0,
            color: Color::srgb(0.9, 0.9, 0.9),
        }
    }
}

fn spawn_button(
    text: &str,
    parent: &mut ChildBuilder,
    image: &Handle<Image>,
    button_tag: ButtonTag,
    font: Handle<Font>,
) {
    parent
        .spawn(ButtonBundle {
            style: ui_components::default_button_style(),
            image: UiImage::from(image.clone()),
            ..default()
        })
        .insert(button_tag)
        .insert(MenuUi)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                ui_components::default_text_style(font),
            ));
        });
}

fn despawn_screen<T: Component>(commands: &mut Commands, entities: &Query<Entity, With<T>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(ui_components::BACKGROUND_COLOR))
        .insert_resource(AppState::MainMenu)
        .add_systems(Startup, setup)
        .add_systems(Update, state_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, _state: ResMut<AppState>) {
    commands.spawn(Camera2dBundle::default());
    setup_main_menu(commands, asset_server)
}

fn state_system(
    mut commands: Commands,
    mut state: ResMut<AppState>,
    asset_server: Res<AssetServer>,
    ui_menu: Query<Entity, With<MenuUi>>,
    mut exit_event_writer: EventWriter<AppExit>,
    interaction_query: Query<(&Interaction, &ButtonTag)>,
) {
    for (interaction, tag) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match *tag {
                ButtonTag::Start => {
                    despawn_screen::<MenuUi>(&mut commands, &ui_menu);
                    *state = AppState::NewScreen;
                }
                ButtonTag::Credits => {
                    despawn_screen::<MenuUi>(&mut commands, &ui_menu);
                    *state = AppState::Credits;
                }
                ButtonTag::Main => {
                    despawn_screen::<MenuUi>(&mut commands, &ui_menu);
                    *state = AppState::MainMenu;
                }
                ButtonTag::Exit => {
                    exit_event_writer.send(AppExit::Success);
                }
            }
        }
    }

    match *state {
        AppState::MainMenu => {
            setup_main_menu(commands, asset_server);
        }
        AppState::NewScreen => {
            setup_new_screen(commands, asset_server);
        }
        AppState::Credits => {
            setup_credits_screen(commands, asset_server);
        }
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("textures/panel-border-010.png");
    let font = asset_server.load("fonts/FiraCode.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },

            ..default()
        })
        .with_children(|parent| {
            spawn_button("Start", parent, &image, ButtonTag::Start, font.clone());
            spawn_button("Credits", parent, &image, ButtonTag::Credits, font.clone());
            spawn_button("Exit", parent, &image, ButtonTag::Exit, font.clone());
        });
}

fn setup_new_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("textures/panel-border-010.png");
    let font = asset_server.load("fonts/FiraCode.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },

            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section("Start Game", TextStyle::default()).with_style(
                        Style {
                            top: Val::Px(12.0),
                            left: Val::Px(12.0),
                            ..default()
                        },
                    ),
                )
                .insert(ButtonTag::Start)
                .insert(MenuUi);
        })
        .with_children(|parent| {
            spawn_button("Voltar", parent, &image, ButtonTag::Main, font.clone());
        });
}

fn setup_credits_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("textures/panel-border-010.png");
    let font = asset_server.load("fonts/FiraCode.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },

            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section("Credits\n", TextStyle::default()).with_style(Style {
                        top: Val::Px(40.0),
                        left: Val::Px(40.0),
                        ..default()
                    }),
                )
                .insert(ButtonTag::Main)
                .insert(MenuUi);
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section("\n", TextStyle::default()).with_style(Style {
                        top: Val::Px(12.0),
                        left: Val::Px(12.0),
                        ..default()
                    }),
                )
                .insert(ButtonTag::Main)
                .insert(MenuUi);
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "\n
                     Design and direction - Jonatas Oliveira\n
                     Development - Jonatas Oliveira\n
                     Sound - Jonatas Oliveira\n
                     Game design - Jonatas Oliveira",
                        TextStyle::default(),
                    )
                    .with_style(Style {
                        top: Val::Px(12.0),
                        left: Val::Px(12.0),
                        ..default()
                    }),
                )
                .insert(ButtonTag::Main)
                .insert(MenuUi);
        })
        .with_children(|parent| {
            spawn_button("Voltar", parent, &image, ButtonTag::Main, font.clone());
        });
}

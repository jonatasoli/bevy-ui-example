use bevy::{prelude::*,};
use crate::app_state::AppState;

pub fn setup_main_menu(mut app: &mut App, mut state: &mut AppState) {
    app.world.set_resource(Scene::current(0)); // Define a cena atual

    // Crie um estilo de texto para o título
    let title_style = TextStyle {
        size: 80.0,
        color: Color::WHITE,
        font: FontDescriptor::default(),
    };

    // Crie um texto para o título
    let title_text = Text::with_style("Jogo Incrível", title_style);

    // Posicione o título na tela
    let title_transform = Transform::from_xyz(0.0, 200.0, 0.0);

    // Crie um texto com instruções
    let instructions_style = TextStyle {
        size: 30.0,
        color: Color::WHITE,
        font: FontDescriptor::default(),
    };

    let instructions_text = Text::with_style("Pressione Enter para iniciar ou Esc para créditos", instructions_style);

    // Posicione as instruções na tela
    let instructions_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    // Crie um botão para iniciar o jogo
    let start_button = Button::new().with_label(Text::with_style("Iniciar", instructions_style));

    // Posicione o botão na tela
    let start_button_transform = Transform::from_xyz(0.0, -100.0, 0.0);

    // Crie um botão para ir para os créditos
    let credits_button = Button::new().with_label(Text::with_style("Créditos", instructions_style));

    // Posicione o botão na tela
    let credits_button_transform = Transform::from_xyz(0.0, -200.0, 0.0);

    // Adicione os elementos à cena
    app
        .add_entity(Text2D::with_bundle(Text2DBundle::from_text(title_text, title_style)))
        .set_transform(title_transform)
        .add_entity(Text2D::with_bundle(Text2DBundle::from_text(instructions_text, instructions_style)))
        .set_transform(instructions_transform)
        .add_entity(start_button.with_id("start_button"))
        .set_transform(start_button_transform)
        .add_entity(credits_button.with_id("credits_button"))
        .set_transform(credits_button_transform);
}

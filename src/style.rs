use stylist::{Style, css};

pub fn title_style() -> Style {
    Style::new(css!(r#"
        font-family: Univers;
        font-weight: bold;
        font-size: 2.5rem;
        text-transform: uppercase;
        color: white;
        text-shadow: 
            0.1rem 0.125rem rgba(0, 0, 0, 1), 0.125rem 0.125rem 1px rgba(0, 0, 0, 0.5);
        user-select: none;
        text-align:center;
        margin-bottom: 1.2rem;
    "#)).expect("Error creating style")
}

pub fn text_style() -> Style {
    Style::new(css!(r#"
        font-family: Univers;
        font-weight: bold;
        font-size: 1.75rem;
        text-transform: capitalize;
        color: #c5c29e;
        text-shadow: 
            0.1rem 0.125rem rgba(0, 0, 0, 1), 0.125rem 0.125rem 1px rgba(0, 0, 0, 0.5);
        user-select: none;
    "#)).expect("Error creating style")
}

pub fn value_style() -> Style {
    Style::new(css!(r#"
        font-family: Univers;
        font-weight: bold;
        font-size: 1.75rem;
        color: white;
        text-shadow: 
            0.1rem 0.125rem rgba(0, 0, 0, 1), 0.125rem 0.125rem 1px rgba(0, 0, 0, 0.5);
        user-select: none;
    "#)).expect("Error creating style")
}

pub fn time_overrun() -> Style {
    Style::new(css!(r#"
        color: #ff1616;
    "#)).expect("Error creating style")
}

pub fn small_gap_left() -> Style {
    Style::new(css!(r#"
        margin-left: 0.2em;
    "#)).expect("Error creating style")
}

pub fn large_gap_right() -> Style {
    Style::new(css!(r#"
        margin-right: 0.7em;
    "#)).expect("Error creating style")
}

pub fn score_container() -> Style {
    Style::new(css!(r#"
        display: flex;
        flex-direction: row;
        flex-wrap: nowrap;
        justify-content: center;
        align-items: center;
    "#)).expect("Error creating style")
}

pub fn container() -> Style {
    Style::new(css!(r#"
        display: flex;
        justify-content: center;
        align-items: center;
        height: 70vh;
        flex-direction: column;
        gap: 5rem;
    "#)).expect("Error creating style")
}

pub fn vitality_style() -> Style {
    Style::new(css!(r#"
        width: 2.5rem;
        height: 2.5rem;
        -webkit-user-select: none;
        pointer-events: none;
    "#)).expect("Error creating style")
}

pub fn depleted() -> Style {
    Style::new(css!(r#"
        filter: brightness(0.4);
    "#)).expect("Error creating style")
}

pub fn blank() -> Style {
    Style::new(css!(r#""#)).expect("Error creating style")
}

pub fn logo_style() -> Style {
    Style::new(css!(r#"
        width: 2em;
        height: 2em;
        color: #fff;
    "#)).expect("Error creating style")
}

pub fn icon_style() -> Style {
    Style::new(css!(r#"
        color: #fff;
        cursor: pointer;
    "#)).expect("Couldn't create delete_style")
}
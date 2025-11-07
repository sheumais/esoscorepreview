use stylist::css;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::{Callback, Event, Html, InputEvent, Properties, TargetCast, classes, function_component, html, use_node_ref, use_state};
use yew_icons::{Icon, IconId};

use crate::{style::*, trials::{Trial, create_trial_structs}};

mod trials;
mod style;

#[derive(Properties, PartialEq)]
pub struct ScoreProps {
    pub trial: Trial,
    pub vitality: u8,
    pub time: u32,
}

#[function_component(ScoreView)]
pub fn score_view(props: &ScoreProps) -> Html {
    let trial = &props.trial;
    let vitality = props.vitality;
    let time = props.time;

    html! {
        <div class={css!("background: repeating-conic-gradient(#5d5d5d 0 25%, #8e8e8e 0 50%) 50% / 20px 20px; padding: 20px;")}>
            <div class={title_style().clone()}>
                { format!("COMPLETED: {}", trial.get_name()) }
            </div>

            <div class={score_container().clone()}>
                <div class={text_style().clone()}>{ "Final Score" }</div>
                <div class={classes!(value_style(), small_gap_left(), large_gap_right())}>
                    { format!("{}", trial.calculate_score_with_vitality(time, vitality)) }
                </div>

                <div class={text_style()}>{ "Total Time" }</div>
                <div class={if time as f64 > trial.get_score_factor() {
                    classes!(value_style(), small_gap_left(), large_gap_right(), time_overrun())
                } else {
                    classes!(value_style(), small_gap_left(), large_gap_right())
                }}>
                    { calculate_time(time) }
                </div>

                <div class={text_style()}>{ "Vitality Bonus" }</div>
                <div class={classes!(value_style(), small_gap_left(), large_gap_right())}>
                    { format!("{}", trial.get_vitality_bonus_with_vitality(vitality)) }
                </div>

                <img
                    class={classes!(
                        vitality_style(),
                        if vitality <= 0 { depleted() } else { blank() }
                    )}
                    src="static/icons/vitalitydepletion.png"
                />
                <div
                    class={classes!(
                        value_style(),
                        small_gap_left(),
                        if vitality <= 0 { depleted() } else { blank() }
                    )}
                >
                    { format!("{}/{}", vitality, trial.get_maximum_vitality()) }
                </div>
            </div>
        </div>
    }
}

fn calculate_time(time: u32) -> String {
    let hours = time / 3_600_000;
    let mut minutes = (time % 3_600_000) / 60_000;
    let mut seconds = (time % 60_000) / 1_000;
    let milliseconds = time % 1_000;

    if milliseconds >= 500 {
        seconds += 1;
        if seconds == 60 {
            seconds = 0;
            if minutes == 59 {
                minutes = 0;
                let hours = hours + 1;
                return format!("{}:{:02}:{:02}", hours, minutes, seconds);
            } else {
                let minutes = minutes + 1;
                return format!("{}:{:02}:{:02}", hours, minutes, seconds);
            }
        }
    }

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("0:{:02}", seconds)
    }
}

#[derive(Properties, PartialEq)]
pub struct TrialSelectorProps {
    pub on_change: Callback<(Trial, u8, u32)>,
}

#[function_component(TrialSelector)]
pub fn trial_selector(props: &TrialSelectorProps) -> Html {
    let trials = create_trial_structs();
    let canvas_ref = use_node_ref();
    let selected_trial = use_state(|| 0usize);
    let vitality = use_state(|| 24u8);
    let total_millis = use_state(|| 900_000u32);

    let on_select_trial = {
        let selected_trial = selected_trial.clone();
        let on_change = props.on_change.clone();
        let vitality = vitality.clone();
        let total_millis = total_millis.clone();
        let trials = trials.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                let idx = input.value().parse::<usize>().unwrap_or(0);
                selected_trial.set(idx);
                let trial = trials[idx].clone();
                let vit = trial.get_maximum_vitality();
                vitality.set(vit);
                let time = trial.get_score_factor();
                total_millis.set(time as u32);
                on_change.emit((trial, vit, time as u32));
            }
        })
    };

    let on_vitality_change = {
        let vitality = vitality.clone();
        let on_change = props.on_change.clone();
        let selected_trial = selected_trial.clone();
        let total_millis = total_millis.clone();
        let trials = trials.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let v = input.value().parse::<u8>().unwrap_or(0);
                vitality.set(v);
                on_change.emit((trials[*selected_trial].clone(), v, *total_millis));
            }
        })
    };

    let time_input = use_state(|| "".to_string());

    let on_time_change = {
        let time_input = time_input.clone();
        let total_millis = total_millis.clone();
        let on_change = props.on_change.clone();
        let selected_trial = selected_trial.clone();
        let vitality = vitality.clone();
        let trials = trials.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                time_input.set(val.clone());

                let parts: Vec<&str> = val.split(':').collect();
                if parts.len() >= 2 {
                    let last = parts.last().unwrap();
                    let (secs, frac_millis) = if let Some((s, frac)) = last.split_once('.') {
                        let secs = s.parse::<u32>().unwrap_or(0);
                        let frac_len = frac.len();
                        let frac_value = frac.parse::<u32>().unwrap_or(0);
                        let millis = match frac_len {
                            0 => 0,
                            1 => frac_value * 100,
                            2 => frac_value * 10,
                            _ => frac_value,
                        };
                        (secs, millis)
                    } else {
                        (last.parse::<u32>().unwrap_or(0), 0)
                    };

                    let mut total_sec = secs;
                    if parts.len() == 2 {
                        let minutes = parts[0].parse::<u32>().unwrap_or(0);
                        total_sec += minutes * 60;
                    } else if parts.len() == 3 {
                        let hours = parts[0].parse::<u32>().unwrap_or(0);
                        let minutes = parts[1].parse::<u32>().unwrap_or(0);
                        total_sec += minutes * 60 + hours * 3600;
                    }

                    let total = total_sec * 1000 + frac_millis;
                    total_millis.set(total);
                    on_change.emit((trials[*selected_trial].clone(), *vitality, total));
                }
            }
        })
    };

    let score_input = use_state(|| "".to_string());

    let on_score_change = {
        let score_input = score_input.clone();
        let total_millis = total_millis.clone();
        let on_change = props.on_change.clone();
        let selected_trial = selected_trial.clone();
        let vitality = vitality.clone();
        let trials = trials.clone();

        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value();
                score_input.set(val.clone());

                if let Ok(final_score) = val.parse::<u32>() {
                    let trial = trials[*selected_trial].clone();
                    let new_time = trial.calculate_time_from_score(final_score, *vitality);
                    total_millis.set(new_time);
                    on_change.emit((trial, *vitality, new_time));
                }
            }
        })
    };

    let render_and_download = {
        let canvas_ref = canvas_ref.clone();
        let total_millis = total_millis.clone();
        let vitality = vitality.clone();
        let trials = trials.clone();
        let selected_trial = selected_trial.clone();

        Callback::from(move |_| {
            let canvas_ref = canvas_ref.clone();
            let total_millis = total_millis.clone();
            let vitality = vitality.clone();
            let trials = trials.clone();
            let selected_trial = selected_trial.clone();

            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let canvas_height = 160;
                canvas.set_height(canvas_height);

                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let trial = trials[*selected_trial].clone();
                let time = *total_millis;
                let vit = *vitality;

                ctx.set_font("bold 40px Univers");
                ctx.set_text_align("center");
                ctx.set_fill_style_str("white");
                ctx.set_shadow_color("rgba(0, 0, 0, 1)");
                ctx.set_shadow_offset_x(2.0);
                ctx.set_shadow_offset_y(2.0);
                ctx.set_shadow_blur(1.0);

                let title_text = format!("COMPLETED: {}", trial.get_name()).to_ascii_uppercase();
                let title_width = ctx.measure_text(&title_text).unwrap().width();

                let score_text = trial.calculate_score_with_vitality(time, vit).to_string();
                let vitality_bonus_text = trial.get_vitality_bonus_with_vitality(vit).to_string();
                let vit_text = format!("{}/{}", vit, trial.get_maximum_vitality());
                let time_text = calculate_time(time);
                let final_score = "Final Score";
                let total_time = "T otal   Time ";
                let total_time_1 = "Total";
                let total_time_2 = " Time";
                let vitality_bonus = "Vitality Bonus";
                let icon_width = 40.0;

                let text_extracts = vec![score_text.clone(), vitality_bonus_text.clone(), vit_text.clone(), time_text.clone(), final_score.to_string(), total_time.to_string(), vitality_bonus.to_string()];
                ctx.set_font("600 28px Univers");
                ctx.set_text_align("left");
                ctx.set_fill_style_str("#c5c29e");
                let total_width: f64 = icon_width + text_extracts.iter().map(|text| ctx.measure_text(&text).unwrap().width()).sum::<f64>();

                let canvas_width = title_width.max(total_width + 81.0) + 10.0;
                canvas.set_width(canvas_width as u32);
                let mut x_pos = (canvas_width - total_width - 80.0) / 2.0;

                ctx.set_font("bold 40px Univers");
                ctx.set_text_align("center");
                ctx.set_fill_style_str("white");
                ctx.set_shadow_color("rgba(0, 0, 0, 1)");
                ctx.set_shadow_offset_x(2.5);
                ctx.set_shadow_offset_y(2.5);
                ctx.set_shadow_blur(1.0);
                ctx.fill_text(&title_text, canvas_width / 2.0 - icon_width / 2.0, 65.0).unwrap();

                ctx.set_font("600 28px Univers");
                ctx.set_text_align("left");
                ctx.set_fill_style_str("#c5c29e");

                ctx.fill_text(&final_score, x_pos, 120.0).unwrap();
                x_pos += ctx.measure_text(&final_score).unwrap().width();
                ctx.save();

                ctx.set_fill_style_str("white");
                x_pos += 5.0;
                ctx.fill_text(&score_text, x_pos, 120.0).unwrap();
                x_pos += ctx.measure_text(&score_text).unwrap().width();
                x_pos += 20.0;
                ctx.restore();
                ctx.save();

                ctx.fill_text(&total_time_1, x_pos, 121.0).unwrap();
                x_pos += ctx.measure_text(&total_time_1).unwrap().width();
                x_pos += 3.0;
                ctx.fill_text(&total_time_2, x_pos, 121.0).unwrap();
                x_pos += ctx.measure_text(&total_time_1).unwrap().width();

                if time as f64 > trial.get_score_factor() {
                    ctx.set_fill_style_str("#ff1616");
                } else {
                    ctx.set_fill_style_str("white");
                }
                x_pos += 10.0;
                ctx.fill_text(&time_text, x_pos, 121.0).unwrap();
                x_pos += ctx.measure_text(&time_text).unwrap().width();
                x_pos += 20.0;
                ctx.restore();

                ctx.fill_text(&vitality_bonus, x_pos, 121.0).unwrap();
                x_pos += ctx.measure_text(&vitality_bonus).unwrap().width();

                ctx.set_fill_style_str("white");
                x_pos += 6.0;
                ctx.fill_text(&vitality_bonus_text, x_pos, 121.0).unwrap();
                x_pos += ctx.measure_text(&vitality_bonus_text).unwrap().width();
                x_pos += 20.0;

                let image_position = x_pos.clone()+1.0;
                x_pos += icon_width;
                x_pos += 6.0;

                let should_filter = if *vitality > 0 {
                    ctx.fill_text(&vit_text, x_pos, 120.0).unwrap();
                    false
                } else {
                    ctx.set_fill_style_str("#666666");
                    ctx.fill_text(&vit_text, x_pos, 120.0).unwrap();
                    true
                };

                ctx.set_shadow_color("transparent");

                let img = HtmlImageElement::new().unwrap();
                img.set_src("static/icons/vitalitydepletion.png");
                let img_clone = img.clone();
                let ctx_clone = ctx.clone();
                let canvas_clone = canvas.clone();
                let trial_clone = trial.clone();

                let onload_closure = Closure::wrap(Box::new(move || {
                    if should_filter {ctx_clone.set_filter("brightness(0.4)");}
                    
                    ctx_clone
                        .draw_image_with_html_image_element_and_dw_and_dh(
                            &img_clone,
                            image_position,
                            91.0,
                            40.0,
                            40.0,
                        )
                        .unwrap();

                    let data_url = canvas_clone.to_data_url_with_type("image/png").unwrap();
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let link = document.create_element("a").unwrap();
                    let link = link.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                    link.set_href(&data_url);
                    link.set_download(&format!(
                        "{}_{}.png",
                        trial_clone.get_raw_name(),
                        trial_clone.calculate_score_with_vitality(time, vit)
                    ));
                    link.click();
                }) as Box<dyn Fn()>);

                img.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                onload_closure.forget();
            }
        })
    };


    let trial_options = trials.iter().enumerate().map(|(i, t)| {
        html! {
            <option value={i.to_string()} selected={i == *selected_trial}>
                { t.get_raw_name() }
            </option>
        }
    });

    let trial = trials[*selected_trial].clone();

    let hours = *total_millis / 3600000;
    let minutes = (*total_millis % 3600000) / 60000;
    let seconds = (*total_millis % 60000) / 1000;
    let millis = *total_millis % 1000;
    let time_str = format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis);
    let score_str = format!("{}", trial.calculate_score_with_vitality(*total_millis, *vitality));

    html! {
        <div class={css!("margin: 0 auto; display: flex; flex-direction: column; align-items: center; row-gap: 1em;")}>
            <select onchange={on_select_trial} class={css!("padding: 2px;")}>
                { for trial_options }
            </select>

            <input
                type="text"
                value={(*time_input).clone()}
                oninput={on_time_change}
                class={css!("width: 7rem; text-align: center;")}
                placeholder={time_str}
            />

            <input
                type="text"
                placeholder={score_str}
                value={(*score_input).clone()}
                oninput={on_score_change}
                class={css!("width: 7rem; text-align: center;")}
            />

            <input
                type="number"
                min="0"
                max={format!("{}", trial.get_maximum_vitality())}
                value={vitality.to_string()}
                oninput={on_vitality_change}
                class={css!("width: 4rem; text-align: center;")}
            />

            <canvas ref={canvas_ref} style="display:none;" />

            <Icon
                class={icon_style().clone()}
                width={"2em"}
                height={"2em"}
                icon_id={IconId::LucideDownload}
                onclick={render_and_download}
            />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let trials = create_trial_structs();
    let trial_state = use_state(|| trials[0].clone());
    let vitality_state = use_state(|| 24u8);
    let time_state = use_state(|| 900_000u32);

    let on_trial_change = {
        let trial_state = trial_state.clone();
        let vitality_state = vitality_state.clone();
        let time_state = time_state.clone();
        Callback::from(move |(trial, vitality, time): (Trial, u8, u32)| {
            trial_state.set(trial);
            vitality_state.set(vitality);
            time_state.set(time);
        })
    };

    html! {
        <div class={container()}>
            <div class={css!("display: flex; gap: 40px; align-items: flex-start; flex-direction: column;")}>
                <TrialSelector on_change={on_trial_change.clone()} />
                <ScoreView trial={(*trial_state).clone()} vitality={vitality_state.min(trial_state.get_maximum_vitality())} time={*time_state} />
            </div>
            <div style="position: fixed; bottom: 1em; right: 1em; display: flex; gap: 1em;">
                <a
                    href={"https://discord.gg/FjJjXHjUQ4"}
                    target="_blank"
                    rel="noopener noreferrer">
                    <Icon icon_id={IconId::BootstrapDiscord} class={logo_style().clone()} />
                </a>
                <a
                    href={"https://github.com/sheumais/esoscorepreview"}
                    target="_blank"
                    rel="noopener noreferrer">
                    <Icon icon_id={IconId::BootstrapGithub} class={logo_style().clone()} />
                </a>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use leptos::*;

fn formatted_time(duration: u32) -> String {
    if duration < 10 {
        format!("0{}", duration)
    } else {
        format!("{}", duration)
    }
}

#[component]
pub fn Timer(current_secs: u32, total_seconds: u32) -> impl IntoView {
    let remaining_secs = total_seconds - current_secs;
    let hours = remaining_secs / (60 * 60);
    let mins = remaining_secs / 60;
    let secs = remaining_secs % 60;
    view! {
        <div>
            <span class="text-6xl text-gray-900 dark:text-white">
                {formatted_time(hours)} : {formatted_time(mins)} : {formatted_time(secs)}
            </span>
        </div>
    }
}

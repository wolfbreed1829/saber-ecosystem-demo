use crate::ui::*;
use leptos::prelude::*;

#[component]
pub fn Entrance(reduced: RwSignal<bool>) -> impl IntoView {
    let aim = RwSignal::new((0.0f64, 0.0f64));
    Effect::new(move |_| {
        if reduced.get() {
            aim.set((0., 0.));
        }
    });
    view! {<section class="landing-scene animated-entrance" aria-label="Saber Tech entrance" style=move ||{let(x,y)=aim.get();format!("--aim-x:{x:.3};--aim-y:{y:.3}")} on:pointermove=move |ev|{if !reduced.get()&&ev.pointer_type()=="mouse"{let w=window().inner_width().ok().and_then(|v|v.as_f64()).unwrap_or(1440.);let h=window().inner_height().ok().and_then(|v|v.as_f64()).unwrap_or(900.);aim.set(((ev.client_x() as f64/w*2.-1.).clamp(-1.,1.),(ev.client_y() as f64/h*2.-1.).clamp(-1.,1.)));}} on:pointerleave=move |_|aim.set((0.,0.))>
        <div class="hero-art"><img src="public/tiger-earth.png" alt="A fierce electric-blue tiger emerging from a luminous planet Earth"/><div class="world-energy" aria-hidden="true"/><div class="orbital orbital-one" aria-hidden="true"/><div class="orbital orbital-two" aria-hidden="true"/><div class="orbital orbital-three" aria-hidden="true"/><div class="hero-particles" aria-hidden="true">{(0..28).map(|i|view!{<i style=format!("left:{}%;top:{}%;--particle-duration:{}s;--particle-delay:-{}s;--particle-size:{}px",35+(i*37)%64,(i*29+7)%100,7+(i%7)*2,(i*13)%21,if i%5==0{3}else{2})/>}).collect_view()}</div><div class="world-light-pass" aria-hidden="true"/></div><div class="hero-grid"/>
        <div class="hero-copy"><p class="eyebrow"><span/>"THE BIG ONE / AN INTEGRATED WORLD"</p><h1>"EVERYTHING."<br/><em>"CONNECTED."</em><br/>"BY DESIGN."</h1><p class="hero-description">"An entire ecosystem. From the places we live to the systems that sustain us. Explore the architecture of a self-reliant Saber Tech world."</p><div class="hero-actions"><a class="primary-button enter-button" href="#/atlas"><span>"ENTER THE ECOSYSTEM"</span><Arrow/></a><a class="hero-interest-link" href="#/interest">"Early investment interest"<span>"↗"</span></a></div><div class="entrance-hint"><span>"01—12"</span>"EXPLORE A COMPLETE WORLD"</div></div>
        <div class="hero-coordinate"><span>"SABER / EARTH SYSTEMS"</span><b>"ONE VISION."<br/>"EVERY ESSENTIAL."</b><i/></div>
        <div class="hero-metrics"><div><b>"24"</b><span>"OPERATING DOMAINS"</span></div><div><b>"12"</b><span>"LAYERS OF EXPLORATION"</span></div><div><b>"01"</b><span>"CONNECTED ECOSYSTEM"</span></div><p>"From everyday services"<br/>"to the ability to produce"<br/>"what keeps them running."</p></div>
    </section>}
}

use crate::ui::*;
use leptos::prelude::*;

// Public delivery configuration. FormSubmit must activate this recipient before delivery.
pub const RECIPIENT: &str = "development@sabertechlogistics.com";
pub const THANK_YOU_URL: &str =
    "https://wolfbreed1829.github.io/saber-ecosystem-demo/#/interest/thanks";

#[component]
pub fn Interest() -> impl IntoView {
    let step = RwSignal::new(0usize);
    let sending = RwSignal::new(false);
    let restore =
        leptos::leptos_dom::helpers::window_event_listener_untyped("pageshow", move |_| {
            sending.set(false)
        });
    on_cleanup(move || restore.remove());
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let organization = RwSignal::new(String::new());
    let location = RwSignal::new(String::new());
    let interest = RwSignal::new("The whole ecosystem".to_string());
    let message = RwSignal::new(String::new());
    view! {<section class="page interest-page"><div class="interest-heading"><div><p class="eyebrow"><span/>"SABER TECH / EARLY INVESTMENT INTEREST"</p><h1>"Be part of what comes next."</h1><p>"Register your interest in a potential future offering as we develop the plans and documentation."</p></div><a class="text-link" href="#/atlas">"Explore the blueprint"<Arrow/></a></div>
    <div class="interest-layout"><aside class="interest-context panel"><div class="interest-art"><img src="public/tiger-earth.png" alt="The electric-blue Saber tiger and Earth"/><div><span class="small-badge">"PLANS IN DEVELOPMENT"</span><h2>"One vision."<br/>"An integrated world."</h2></div></div><div class="interest-context-copy"><p>"Homes, healthcare, learning, food, energy, industry, and the systems that connect them. Start a conversation with the development team about the Saber Tech vision."</p><div class="interest-stages"><span class="current"><i/>"Develop the plans"</span><span><i/>"Prepare the offering"</span><span><i/>"Potential launch"</span></div><div class="interest-notice" id="interest-notice"><h3>"EARLY INTEREST · NO COMMITMENT"</h3><p>"Saber Tech is considering an offering of securities exempt from registration. A specific exemption and offering terms have not yet been determined."</p><p>"We are not requesting money or other consideration and will not accept any sent in response. Offers to buy cannot be accepted, and purchase payments cannot be received, until an exemption is selected and its applicable filing, disclosure, or qualification requirements are met."</p><p>"Your indication of interest creates no obligation or commitment of any kind and does not reserve an allocation."</p></div></div></aside>
    <form class="interest-form panel" action=format!("https://formsubmit.co/{RECIPIENT}") method="POST" on:submit=move |ev|{if step.get()==0{ev.prevent_default();step.set(1);}else{sending.set(true);}}>
        <input type="hidden" name="_subject" value="Saber Tech — new early investment interest"/>
        <input type="hidden" name="_template" value="table"/>
        <input type="hidden" name="_next" value=THANK_YOU_URL/>
        <input type="hidden" name="name" prop:value=move ||name.get()/>
        <input type="hidden" name="email" prop:value=move ||email.get()/>
        <input type="hidden" name="Organization" prop:value=move ||organization.get()/>
        <input type="hidden" name="Location" prop:value=move ||location.get()/>
        <input type="hidden" name="Primary interest" prop:value=move ||interest.get()/>
        <input type="hidden" name="Message" prop:value=move ||message.get()/>
        <input type="hidden" name="Submission type" value="Nonbinding early investment interest; no money, allocation, or commitment"/>
        <input type="hidden" name="Consent version" value="2026-10-27 / contact permission and nonbinding-interest acknowledgment"/>
        <input class="form-honeypot" type="text" name="_honey" tabindex="-1" autocomplete="off" aria-hidden="true"/>
        <div class="interest-form-top"><div><span class="technical-label">"START THE CONVERSATION"</span><h2>{move ||if step.get()==0{"Tell us about your interest."}else{"Review before sending."}}</h2></div><span class="form-step">{move ||format!("0{}",step.get()+1)}<small>" / 02"</small></span></div>
        <div class="form-progress" aria-label="Form progress"><span class:active=move ||step.get()==0>"01 / Contact & interests"</span><span class:active=move ||step.get()==1>"02 / Review & send"</span></div>
        {move ||if step.get()==0{view!{<div class="interest-fields"><div class="form-field"><label for="interest-name">"Full name "<b>"*"</b></label><input id="interest-name" type="text" autocomplete="name" required maxlength="120" placeholder="Your name" prop:value=move ||name.get() on:input=move |ev|name.set(event_target_value(&ev))/></div><div class="form-field"><label for="interest-email">"Email address "<b>"*"</b></label><input id="interest-email" type="email" autocomplete="email" required maxlength="254" placeholder="you@example.com" prop:value=move ||email.get() on:input=move |ev|email.set(event_target_value(&ev))/></div><div class="form-field"><label for="interest-organization">"Organization "<small>"optional"</small></label><input id="interest-organization" type="text" autocomplete="organization" maxlength="160" placeholder="Company or organization" prop:value=move ||organization.get() on:input=move |ev|organization.set(event_target_value(&ev))/></div><div class="form-field"><label for="interest-location">"Location "<small>"optional"</small></label><input id="interest-location" type="text" maxlength="160" placeholder="City, state / country" prop:value=move ||location.get() on:input=move |ev|location.set(event_target_value(&ev))/></div><div class="form-field wide"><label for="interest-topic">"What interests you most?"</label><select id="interest-topic" prop:value=move ||interest.get() on:change=move |ev|interest.set(event_target_value(&ev))>{["The whole ecosystem","Homes, healthcare & family life","Food, energy & essential infrastructure","Industry, manufacturing & logistics","Software, coordination & technology"].into_iter().map(|topic|view!{<option value=topic>{topic}</option>}).collect_view()}</select></div></div><div class="form-actions"><span>"* Required fields"</span><button class="primary-button" type="submit">"Review your interest"<Arrow/></button></div>}.into_any()}else{view!{<div class="interest-review" aria-live="polite"><dl><div><dt>"NAME"</dt><dd>{move ||name.get()}</dd></div><div><dt>"EMAIL"</dt><dd>{move ||email.get()}</dd></div><div><dt>"ORGANIZATION"</dt><dd>{move ||if organization.get().trim().is_empty(){"Not provided".into()}else{organization.get()}}</dd></div><div><dt>"LOCATION"</dt><dd>{move ||if location.get().trim().is_empty(){"Not provided".into()}else{location.get()}}</dd></div><div class="wide"><dt>"PRIMARY INTEREST"</dt><dd>{move ||interest.get()}</dd></div></dl><div class="review-note-field"><div class="form-field wide"><label for="interest-message">"What would you like to discuss? "<small>"optional"</small></label><textarea id="interest-message" rows="3" maxlength="1500" placeholder="Tell us what you would like to understand about the plans or a potential future offering." prop:value=move ||message.get() on:input=move |ev|message.set(event_target_value(&ev))/></div></div></div><label class="interest-consent"><input type="checkbox" name="Contact consent" value="Agreed to contact and understood nonbinding interest" required aria-describedby="interest-notice"/><span>"I agree to be contacted by Saber Tech about my interest. I understand this is a nonbinding indication of interest, with no obligation or commitment."</span></label><div class="form-actions"><button type="button" class="text-button" disabled=move ||sending.get() on:click=move |_|step.set(0)>"← Edit details"</button><button class="primary-button" type="submit" disabled=move ||sending.get()>{move ||if sending.get(){"Opening secure submission…"}else{"Send my interest"}}<Arrow/></button></div>}.into_any()}}
        <p class="interest-privacy">"Your details are sent to "<a href=format!("mailto:{RECIPIENT}")>{RECIPIENT}</a>" through "<a href="https://formsubmit.co/privacy.pdf" target="_blank" rel="noopener noreferrer">"FormSubmit"</a>". You may be asked to complete its spam check. To withdraw your interest or request removal, contact the same address."</p>
    </form></div></section>}
}

#[component]
pub fn InterestThanks() -> impl IntoView {
    view! {<section class="page interest-thanks"><div class="thank-you-card panel"><span class="confirmation-mark" aria-hidden="true">"✓"</span><p class="eyebrow">"SABER TECH / EARLY INTEREST"</p><h1>"Thank you for starting"<br/>"the conversation."</h1><p>"Your form has been submitted. The development team can follow up using the contact details you provided. This creates no investment obligation, commitment, or allocation."</p><a class="primary-button" href="#/atlas">"Continue exploring"<Arrow/></a><a class="text-link" href=format!("mailto:{RECIPIENT}")>{RECIPIENT}</a></div></section>}
}

//! Project links: the Help menu, the About dialog and the home screen open PdfKu's own pages, and
//! nothing in the app points at the project it was copied from.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::links;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

fn harness(setup: impl FnOnce(&mut PdfCraftApp) + 'static) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

/// PdfKu is its own project: the brand, the chat server and the site of the project it started
/// from are not its to show (their licence says a modified version must remove them).
#[test]
fn nothing_points_at_the_project_pdfku_was_copied_from() {
    for l in links::LINKS {
        assert!(l.url.starts_with("https://github.com/andriyanto/pdfku"), "{}", l.url);
    }
    let h = harness(|_| {});
    for gone in ["Discord", "Join our Discord", "ArtCraft", "ArtCraft website", "Join the ArtCraft community"] {
        assert!(h.query_by_label(gone).is_none(), "{gone} is still on the home screen");
    }
}

/// The community button opens the README at its community section, which holds the WhatsApp
/// invite; the section's heading is what GitHub turns into the `#komunitas` anchor, so renaming
/// it would leave the button opening the top of the README.
#[test]
fn the_community_link_lands_on_the_readmes_community_section() {
    let readme = include_str!("../../../../README.md");
    let anchor = links::COMMUNITY.rsplit_once('#').map(|(_, a)| a).unwrap_or_default();
    assert_eq!(anchor, "komunitas");
    assert!(readme.lines().any(|l| l == "## Komunitas"), "README.md has no “## Komunitas” heading");
}

#[test]
fn home_screen_links() {
    for (label, url) in [
        ("Join the WhatsApp group", "https://github.com/andriyanto/pdfku#komunitas"),
        ("Discussions", "https://github.com/andriyanto/pdfku/discussions"),
        ("PdfKu on GitHub", "https://github.com/andriyanto/pdfku"),
        ("PdfKu web page", "https://github.com/andriyanto/pdfku#readme"),
        ("Report a problem", "https://github.com/andriyanto/pdfku/issues"),
    ] {
        let mut h = harness(|_| {});
        h.get_by_label("PdfKu is open source");
        h.get_by_label(label).click();
        h.run_steps(2);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(url), "{label}");
    }
}

#[test]
fn home_community_card_closes_for_this_run() {
    let mut h = harness(|_| {});
    h.get_by_label("PdfKu is open source");
    h.get_by_label("Close").click();
    h.run_steps(2);
    assert!(h.state().community_card_closed);
    assert!(h.query_by_label("PdfKu is open source").is_none());
    assert!(h.query_by_label("Report a problem").is_none());
    // Not remembered: a fresh app shows the card again.
    assert!(!PdfCraftApp::new().community_card_closed);
}

#[test]
fn about_dialog_shows_the_app_and_its_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let mut h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    assert!(h.query_all_by_label("PdfKu").count() >= 2, "the icon (alt text) and the name");
    assert!(h.query_by_label("ArtCraft").is_none(), "the brand of the project PdfKu was copied from is gone");
    // About says what PdfKu is made from and who modifies it, on its first tab and above the
    // contributors, who are the original project's.
    h.get_by_label("PdfKu is a modified version of");
    h.get_by_label("PdfCraft");
    h.get_by_label("@andriyanto");
    h.get_by_label("Report a problem").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::ISSUES));
    h.get_by_label("Join the WhatsApp group").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::COMMUNITY));
    settled(&mut h, "Contributors").click();
    settled(&mut h, "@andriyanto");
    h.get_by_label("PdfKu is a modified version of");
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(pdfcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
fn settled<'a>(h: &'a mut Harness<'static, PdfCraftApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Contributors").click();
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    settled(&mut h, "@echelon");
    settled(&mut h, "Table").click();
    settled(&mut h, "PRs");
    settled(&mut h, "Display name").click();
    settled(&mut h, "Brandon Thomas");
    settled(&mut h, "Models").click();
    h.run_steps(4);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}

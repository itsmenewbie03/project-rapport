use crate::utils::models::FeedbackDataRow;
use crate::FeedbackData;
use genpdf::{elements, fonts, style};
use genpdf::{Alignment, Element};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];
const DEFAULT_FONT_NAME: &str = "LiberationSans";

// BUG: the code below is a 100% dogshit
// only dumb people will solve problems in this way
// and I'm one of them
/// Returns bool if the pdf is generated successfully
pub async fn generate_pdf(feedback_data: Vec<FeedbackDataRow>) -> Option<String> {
    // NOTE: prepare the document
    let font_dir = FONT_DIRS
        .iter()
        .find(|path| std::path::Path::new(path).exists())
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");
    let mut doc = genpdf::Document::new(default_font);

    // INFO: fetch the data
    if feedback_data.is_empty() {
        return None;
    }
    // INFO: prepare the data for SUMMARY table
    let total = feedback_data.len();
    let mut negative = 0;
    let mut neutral = 0;
    let mut positive = 0;
    feedback_data.iter().for_each(|x| match x.tag.as_str() {
        "negative" => negative += 1,
        "neutral" => neutral += 1,
        "positive" => positive += 1,
        _ => {
            println!("[RUST] Unknown tag: {}", x.tag);
        }
    });
    // INFO: prepare the data for DETAILS table
    let perfect_score = total * 5;
    let mut access_and_facilities: i32 = 0;
    let mut assurance: i32 = 0;
    let mut communication: i32 = 0;
    let mut integrity: i32 = 0;
    let mut outcome: i32 = 0;
    let mut overall_satisfaction: i32 = 0;
    let mut reliability: i32 = 0;
    let mut responsiveness: i32 = 0;
    let mut value_for_money: i32 = 0;
    feedback_data.iter().for_each(|x| {
        // BUG: this is dumb, I know
        let data = &x.data;
        let start_idx = data.find("{\"feedback_data\":");
        let end_idx = data.find(",\"emotion_data\"");
        match (start_idx, end_idx) {
            (Some(s), Some(e)) => {
                let fdata = &data[s + 17..e];
                let feedback_data = FeedbackData::parse(fdata).unwrap();
                access_and_facilities += feedback_data.access_and_facilities as i32;
                assurance += feedback_data.assurance as i32;
                communication += feedback_data.communication as i32;
                integrity += feedback_data.integrity as i32;
                outcome += feedback_data.outcome as i32;
                overall_satisfaction += feedback_data.overall_satisfaction as i32;
                reliability += feedback_data.reliability as i32;
                responsiveness += feedback_data.responsiveness as i32;
                value_for_money += feedback_data.value_for_money as i32;
            }
            _ => {
                print!("[RUST] Failed to parse feedback data!");
            }
        }
    });
    doc.set_title("Feedback Data Report");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc.push(
        elements::Paragraph::new("Feedback Data Report")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    doc.push(elements::Break::new(0.5));
    doc.push(elements::Paragraph::new("Summary").styled(style::Style::new().bold()));
    doc.push(elements::Break::new(0.5));

    let mut table = elements::TableLayout::new(vec![2, 5, 5]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    table
        .row()
        .element(
            elements::Paragraph::new("Category")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Total")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Percentage")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .push()
        .expect("Invalid table row");
    table
        .row()
        .element(elements::Paragraph::new("Positive").padded(1))
        .element(elements::Paragraph::new(positive.to_string()).padded(1))
        .element(
            elements::Paragraph::new(format!(
                "{:.2}%",
                (positive as f64 / total as f64 * 100_f64)
            ))
            .padded(1),
        )
        .push()
        .expect("Invalid table row");

    table
        .row()
        .element(elements::Paragraph::new("Neutral").padded(1))
        .element(elements::Paragraph::new(neutral.to_string()).padded(1))
        .element(
            elements::Paragraph::new(format!("{:.2}%", (neutral as f64 / total as f64 * 100_f64)))
                .padded(1),
        )
        .push()
        .expect("Invalid table row");

    table
        .row()
        .element(elements::Paragraph::new("Negative").padded(1))
        .element(elements::Paragraph::new(negative.to_string()).padded(1))
        .element(
            elements::Paragraph::new(format!(
                "{:.2}%",
                (negative as f64 / total as f64 * 100_f64)
            ))
            .padded(1),
        )
        .push()
        .expect("Invalid table row");
    table
        .row()
        .element(elements::Paragraph::new("").padded(1))
        .element(elements::Paragraph::new("").padded(1))
        .element(
            elements::Paragraph::new(format!("Total Feedbacks: {}", total))
                .styled(style::Style::new().bold())
                .padded(1),
        )
        .push()
        .expect("Invalid table row");
    doc.push(table);

    doc.push(elements::Break::new(0.5));
    doc.push(elements::Paragraph::new("Details").styled(style::Style::new().bold()));
    doc.push(elements::Break::new(0.5));
    let mut table = elements::TableLayout::new(vec![3, 5, 5]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    table
        .row()
        .element(
            elements::Paragraph::new("Aspect")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Score")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Percentage")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .push()
        .expect("Invalid table row");

    let scores: Vec<i32> = vec![
        access_and_facilities,
        assurance,
        communication,
        integrity,
        outcome,
        overall_satisfaction,
        reliability,
        responsiveness,
        value_for_money,
    ];

    let aspects = [
        "Access and Facilities",
        "Assurance",
        "Communication",
        "Integrity",
        "Outcome",
        "Overall Satisfaction",
        "Reliability",
        "Responsiveness",
        "Value for Money",
    ];

    scores.iter().enumerate().for_each(|(idx, score)| {
        table
            .row()
            .element(elements::Paragraph::new(aspects[idx]).padded(1))
            .element(elements::Paragraph::new(format!("{}/{}", score, perfect_score)).padded(1))
            .element(
                elements::Paragraph::new(format!(
                    "{:.2}%",
                    (*score as f64 / perfect_score as f64) * 100_f64
                ))
                .padded(1),
            )
            .push()
            .expect("Invalid table row");
    });
    table
        .row()
        .element(elements::Paragraph::new("").padded(1))
        .element(elements::Paragraph::new("").padded(1))
        .element(
            elements::Paragraph::new(format!("Total Feedbacks: {}", total))
                .styled(style::Style::new().bold())
                .padded(1),
        )
        .push()
        .expect("Invalid table row");
    doc.push(table);
    let cwd = std::env::current_dir().unwrap();
    let fname = format!("{}/report.pdf", cwd.display());
    doc.render_to_file(&fname)
        .expect("Failed to write output file");
    Some(fname)
}

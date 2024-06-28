// @ts-ignore
import HRNumbers from 'human-readable-numbers';

type FeedbackType = "trad" | "hybrid";
const parse_feedback_data = (feedback_type: FeedbackType, data: string) => {
  return feedback_type == "trad" ? trad_parser(data) : hybrid_parser(data);
};

const trad_parser = (data: string) => { };

const hybrid_parser = (data: string) => {
  let output: object[] = [];
  let parsed_data = JSON.parse(data);
  parsed_data.forEach((feedback: any) => {
    const feedback_data = JSON.parse(feedback.data);
    let temp = {
      timestamp: feedback.created_at,
      ...feedback_data,
    };
    output.push(temp);
  });
  return vizzu_parser(output);
};

// BUG: this code is a piece of shit
// i'm too dumb to come up with a clean solution
// maybe I'll rewrite this in Rust (soon or never xD)
// R.I.P RAM
const vizzu_parser = (data: object[]) => {
  let arr = [
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

  let idx_table = arr.map((data) => data.replaceAll(" ", "").toLowerCase());
  let scores: number[] = new Array(9).fill(0);
  let perfect_scores: number[] = new Array(9).fill(data.length * 5);

  let emotions = [
    "Angry",
    "Disgust",
    "Fear",
    "Happy",
    "Neutral",
    "Sad",
    "Surprise",
  ];

  let emotion_idx_table = emotions.map((data) => data.toLowerCase());
  let emotion_data_aggregate: Record<string, number[]> = {};

  idx_table.forEach((e) => {
    emotion_data_aggregate[e] = new Array(7).fill(0);
  });
  data.forEach((e: any) => {
    const feedback_data = e.feedback_data;
    for (const key in feedback_data) {
      if (feedback_data.hasOwnProperty(key)) {
        const element = feedback_data[key];
        const idx = idx_table.indexOf(key.replaceAll("_", ""));
        scores[idx] += element;
      }
    }
    const emotion_data = e.emotion_data;
    for (const key in emotion_data) {
      if (emotion_data.hasOwnProperty(key)) {
        const actual_key = key.replaceAll("_", "");
        const element = emotion_data[key];
        const idx = emotion_idx_table.indexOf(element);
        emotion_data_aggregate[actual_key][idx] += 1;
      }
    }
  });
  let vizzu_data = {
    series: [
      { name: "Quality", values: arr },
      { name: "Total Rating", values: scores },
      { name: "Perfect Scores", values: perfect_scores },
      { name: "Emotion", values: emotions },
      {
        name: "Access and Facilities",
        values: emotion_data_aggregate["accessandfacilities"],
      },
      { name: "Assurance", values: emotion_data_aggregate["assurance"] },
      {
        name: "Communication",
        values: emotion_data_aggregate["communication"],
      },
      { name: "Integrity", values: emotion_data_aggregate["integrity"] },
      { name: "Outcome", values: emotion_data_aggregate["outcome"] },
      {
        name: "Overall Satisfaction",
        values: emotion_data_aggregate["overallsatisfaction"],
      },
      { name: "Reliability", values: emotion_data_aggregate["reliability"] },
      {
        name: "Responsiveness",
        values: emotion_data_aggregate["responsiveness"],
      },
      {
        name: "Value for Money",
        values: emotion_data_aggregate["valueformoney"],
      },
    ],
  };
  return vizzu_data;
};

const stats_parser = (data: string): Record<string, number> => {
  const parsed_data = JSON.parse(data);
  let feedback_stats: Record<string, number> = {
    "positive": 0,
    "negative": 0,
    "neutral": 0,
    "total": parsed_data.length,
  };
  parsed_data.forEach((e: any) => {
    const parsed_inner_data = JSON.parse(e.data);
    const { metadata } = parsed_inner_data;
    const { feedback_category } = metadata;
    feedback_stats[feedback_category] += 1;
  });
  // TODO: make human readable numbers
  for (const key in feedback_stats) {
    if (feedback_stats.hasOwnProperty(key)) {
      const element = feedback_stats[key];
      feedback_stats[key] = HRNumbers.toHumanString(element);
    }
  }
  return feedback_stats;
}

export { parse_feedback_data, stats_parser };

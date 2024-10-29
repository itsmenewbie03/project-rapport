// @ts-ignore
import HRNumbers from 'human-readable-numbers';
type FeedbackType = 'trad' | 'hybrid';

const parse_feedback_data = (feedback_type: FeedbackType, data: string) => {
  return feedback_type == 'trad' ? trad_parser(data) : hybrid_parser(data);
};

// WARN: this code is high voltage xD
// DO NOT TOUCH YOU WILL BE FIRED !!!!!!!
const getMaxKey = (
  obj: Record<string, number>,
): [string, number] | undefined => {
  // Check if the object is empty
  if (Object.keys(obj).length === 0) {
    return undefined;
  }

  // Find the entry with the max value using reduce and destructuring
  return Object.entries(obj).reduce(
    ([maxKey, maxValue], [key, value]) =>
      value > maxValue ? [key, value] : [maxKey, maxValue],
    // Initial value for reduction (first key-value pair)
    [Object.keys(obj)[0], Object.values(obj)[0]],
  );
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
    'Access and Facilities',
    'Assurance',
    'Communication',
    'Integrity',
    'Outcome',
    'Overall Satisfaction',
    'Reliability',
    'Responsiveness',
    'Value for Money',
  ];

  let idx_table = arr.map((data) => data.replaceAll(' ', '').toLowerCase());
  let scores: number[] = new Array(9).fill(0);
  let perfect_scores: number[] = new Array(9).fill(data.length * 5);

  let emotions = [
    'Angry',
    'Disgust',
    'Fear',
    'Happy',
    'Neutral',
    'Sad',
    'Surprise',
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
        const idx = idx_table.indexOf(key.replaceAll('_', ''));
        scores[idx] += element;
      }
    }
    const emotion_data = e.emotion_data;
    for (const key in emotion_data) {
      if (emotion_data.hasOwnProperty(key)) {
        const actual_key = key.replaceAll('_', '');
        const element = emotion_data[key];
        const idx = emotion_idx_table.indexOf(element);
        emotion_data_aggregate[actual_key][idx] += 1;
      }
    }
  });

  let vizzu_data = {
    series: [
      { name: 'Quality', values: arr },
      { name: 'Total Rating', values: scores },
      { name: 'Perfect Scores', values: perfect_scores },
      { name: 'Emotion', values: emotions },
      {
        name: 'Access and Facilities',
        values: emotion_data_aggregate['accessandfacilities'],
      },
      { name: 'Assurance', values: emotion_data_aggregate['assurance'] },
      {
        name: 'Communication',
        values: emotion_data_aggregate['communication'],
      },
      { name: 'Integrity', values: emotion_data_aggregate['integrity'] },
      { name: 'Outcome', values: emotion_data_aggregate['outcome'] },
      {
        name: 'Overall Satisfaction',
        values: emotion_data_aggregate['overallsatisfaction'],
      },
      { name: 'Reliability', values: emotion_data_aggregate['reliability'] },
      {
        name: 'Responsiveness',
        values: emotion_data_aggregate['responsiveness'],
      },
      {
        name: 'Value for Money',
        values: emotion_data_aggregate['valueformoney'],
      },
    ],
  };
  return vizzu_data;
};

const stats_parser = (data: string): Record<string, number> => {
  const parsed_data = JSON.parse(data);
  let feedback_stats: Record<string, number> = {
    positive: 0,
    negative: 0,
    neutral: 0,
    total: parsed_data.length,
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
};

const quantify_emotion = (emotion: string): number => {
  const emotion_map: Record<string, number> = {
    angry: -1,
    fear: -1,
    neutral: 0,
    sad: -1,
    disgust: -1,
    happy: 1,
    surprise: -1,
  };
  if (emotion_map.hasOwnProperty(emotion)) {
    return emotion_map[emotion];
  }
  console.log(`Unknown emotion detected: ${emotion}`);
  // NOTE: we're setting unknown emotions to neutral
  // this should be a weird case
  return 0;
};

const calc_final_rating = (
  actual_rating: number,
  emotion_rating: string,
  emotion_weight: number,
): number => {
  const emotion_rating_value = quantify_emotion(emotion_rating);
  // INFO: min-max normalization because JS sucks xD (ironic coz i'm writing this in TS xD)
  const normalized_emotion_rating = (emotion_rating_value - -1) / (1 - -1);
  const min_value = 1 - emotion_weight * 1;
  const max_value = 5 - emotion_weight * 5 + emotion_weight;
  const new_max = 5;
  const new_min = 1;
  const combined_rating =
    normalized_emotion_rating * emotion_weight +
    actual_rating * (1 - emotion_weight);
  // INFO: normalize combined_rating using min-max normalization
  const final_rating =
    ((combined_rating - min_value) / (max_value - min_value)) *
    (new_max - new_min) +
    new_min;
  return final_rating;
};

// TEST: let's see if we did it right first try xD
// UPDATE: i acutally did it right, first try xD
// can you do that? you dumb shit dev xD
// function testCalcFinalRating() {
//   const testCases = [
//     {
//       actual_rating: 1,
//       emotion_rating: 'sad',
//       emotion_weight: 0.5,
//       expected_rating: 1.0,
//     },
//     {
//       actual_rating: 2,
//       emotion_rating: 'sad',
//       emotion_weight: 0.5,
//       expected_rating: 1.8,
//     },
//     {
//       actual_rating: 3,
//       emotion_rating: 'sad',
//       emotion_weight: 0.5,
//       expected_rating: 2.6,
//     },
//     {
//       actual_rating: 4,
//       emotion_rating: 'sad',
//       emotion_weight: 0.5,
//       expected_rating: 3.4,
//     },
//     {
//       actual_rating: 5,
//       emotion_rating: 'sad',
//       emotion_weight: 0.5,
//       expected_rating: 4.2,
//     },
//     {
//       actual_rating: 1,
//       emotion_rating: 'neutral',
//       emotion_weight: 0.5,
//       expected_rating: 1.4,
//     },
//     {
//       actual_rating: 2,
//       emotion_rating: 'neutral',
//       emotion_weight: 0.5,
//       expected_rating: 2.2,
//     },
//     {
//       actual_rating: 3,
//       emotion_rating: 'neutral',
//       emotion_weight: 0.5,
//       expected_rating: 3.0,
//     },
//     {
//       actual_rating: 4,
//       emotion_rating: 'neutral',
//       emotion_weight: 0.5,
//       expected_rating: 3.8,
//     },
//     {
//       actual_rating: 5,
//       emotion_rating: 'neutral',
//       emotion_weight: 0.5,
//       expected_rating: 4.6,
//     },
//     {
//       actual_rating: 1,
//       emotion_rating: 'happy',
//       emotion_weight: 0.5,
//       expected_rating: 1.8,
//     },
//     {
//       actual_rating: 2,
//       emotion_rating: 'happy',
//       emotion_weight: 0.5,
//       expected_rating: 2.6,
//     },
//     {
//       actual_rating: 3,
//       emotion_rating: 'happy',
//       emotion_weight: 0.5,
//       expected_rating: 3.4,
//     },
//     {
//       actual_rating: 4,
//       emotion_rating: 'happy',
//       emotion_weight: 0.5,
//       expected_rating: 4.2,
//     },
//     {
//       actual_rating: 5,
//       emotion_rating: 'happy',
//       emotion_weight: 0.5,
//       expected_rating: 5.0,
//     },
//   ];
//
//   for (const testCase of testCases) {
//     const actualRating = calc_final_rating(
//       testCase.actual_rating,
//       testCase.emotion_rating,
//       testCase.emotion_weight,
//     );
//
//     assert.equal(
//       actualRating,
//       testCase.expected_rating,
//       `Incorrect final rating for ${JSON.stringify(testCase)}`,
//     );
//   }
// }

const history_parser = (data: string, emotion_weight: number): any => {
  let output: object[] = [];
  const parsed_data = JSON.parse(data);
  // INFO: manual parsing it is xD
  parsed_data.forEach((e: any) => {
    const inner_parsed_data = JSON.parse(e.data);
    let total_rating = 0;
    const { metadata, feedback_data } = inner_parsed_data;
    for (const key in feedback_data) {
      if (feedback_data.hasOwnProperty(key)) {
        const element = feedback_data[key];
        total_rating += element;
      }
    }
    let mean_rating = total_rating / 9;
    let mean_rating_percentage = (mean_rating / 5.0) * 100;
    // INFO: emotion data parsing (this easy to do in pandas xD)
    // but I will write it by hand because I'm dumb (don't be like me xD)
    const { emotion_data } = inner_parsed_data;
    let emotion_data_aggregate: Record<string, number> = {};
    for (const key in emotion_data) {
      if (emotion_data.hasOwnProperty(key)) {
        const element = emotion_data[key];
        if (Object.keys(emotion_data_aggregate).includes(element)) {
          emotion_data_aggregate[element] += 1;
          continue;
        }
        emotion_data_aggregate[element] = 1;
      }
    }
    // @ts-ignore
    const [max_key, max_value] = getMaxKey(emotion_data_aggregate);
    const dominant_emotion = max_key.charAt(0).toUpperCase() + max_key.slice(1);
    const dominant_emotion_percentage = (max_value / 9) * 100;
    const final_rating = calc_final_rating(
      mean_rating,
      // NOTE: the dominant emotion is a ucfirst'd string
      max_key,
      emotion_weight,
    );
    let temp = {
      id: e.id,
      tag: e.tag,
      office_name: metadata.office_name,
      mean_rating: mean_rating.toFixed(2),
      mean_rating_percent: mean_rating_percentage.toFixed(0),
      overall_emotion: dominant_emotion,
      overall_emotion_percent: dominant_emotion_percentage.toFixed(0),
      final_rating: final_rating.toFixed(2),
      ...inner_parsed_data,
    };
    output.push(temp);
  });
  return output;
};

export { parse_feedback_data, stats_parser, history_parser };

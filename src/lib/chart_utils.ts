import {
  fromUnixTime,
  getDate,
  getDaysInMonth,
  getISOWeeksInYear,
  getMonth,
  getWeek,
} from 'date-fns';

export type ReportType = 'daily' | 'weekly' | 'monthly';

type ChartData = {
  labels: string[];
  datasets: any[];
};

const get_data_array = (report_type: ReportType) => {
  switch (report_type) {
    case 'daily':
      return Array.from({ length: 32 }, () => 0);
    case 'weekly':
      return Array.from({ length: 53 }, () => 0);
    case 'monthly':
      return Array.from({ length: 12 }, () => 0);
    default:
      throw new Error('Invalid report type');
  }
};

const pretty_dataset = (dataset: any[]): any[] => {
  return dataset.map((elem) => {
    return {
      ...elem,
      label: elem.label
        .replaceAll('_', ' ')
        .split(' ')
        .map((word: string) => {
          if (['for', 'and'].includes(word)) return word;
          return word.charAt(0).toUpperCase() + word.slice(1);
        })
        .join(' '),
    };
  });
};

const generate_dataset = (
  report_type: ReportType,
  label: string,
  idx: number,
  average: number,
) => {
  const dataset = {
    label,
    lineTension: 0.1,
    borderDash: [],
    borderDashOffset: 0.0,
    pointBorderWidth: 5,
    pointRadius: 2,
    pointHitRadius: 10,
    pointStyle: 'rectRot',
    data: get_data_array(report_type),
  };
  dataset.data[idx] = average;
  return dataset;
};

const generate_weekly_report = (data: string): ChartData => {
  const dataset: any[] = [];
  const jdata = JSON.parse(data);
  // TODO: let's get the number of weeks of the
  // year based on the first entry so we can set the labels
  // with the each week of the year
  const { created_at } = jdata[0];
  const number_weeks = getISOWeeksInYear(fromUnixTime(created_at));
  // NOTE: this map contains the feedback data grouped by week
  // NOTE: the key is the week and the value is an array of feedback data
  const grouped_data: Map<number, any> = new Map();
  jdata.forEach((elem: any) => {
    const { created_at, data } = elem;
    // NOTE: datefns is ISO compliant
    // so if it seems inacurate, go complain to ISO
    const week = getWeek(fromUnixTime(created_at));
    const { feedback_data } = JSON.parse(data);
    if (grouped_data.has(week)) {
      // NOTE: i hate ...spread
      grouped_data.set(week, [...grouped_data.get(week), feedback_data]);
    } else {
      grouped_data.set(week, [feedback_data]);
    }
  });

  const x_labels = [...Array(number_weeks).keys()].map((v) =>
    (v + 1).toString(),
  );

  // TODO: get the average of the feedback data for each week
  grouped_data.forEach((data, week) => {
    const keys = Object.keys(data[0]);
    const total = data.length;
    // TODO: compute the average for each feedback category
    const avg = keys.map((key) => {
      const sum = data.reduce((acc: number, elem: any) => {
        return acc + elem[key];
      }, 0);
      return [key, sum / total];
    });
    avg.forEach((elem) => {
      const [key, val] = elem;
      // TODO: check if the dataset already contains the key
      // if it does, update the data field
      const pos_idx = dataset.findIndex((elem: any) => elem.label === key);
      if (pos_idx !== -1) {
        dataset[pos_idx].data[week - 1] = val;
        return;
      } else {
        dataset.push(
          generate_dataset('weekly', key as string, week - 1, val as number),
        );
      }
    });
  });
  // TODO: make the keys pretty
  return {
    labels: x_labels,
    datasets: pretty_dataset(dataset),
  };
};

const generate_daily_report = (data: string): ChartData => {
  const dataset: any[] = [];
  const jdata = JSON.parse(data);
  // TODO: let's get the current number of days of the
  // month based on the first entry so we can set the labels
  // with the each day of the month
  const { created_at } = jdata[0];
  const number_days = getDaysInMonth(fromUnixTime(created_at));
  // NOTE: this map contains the feedback data grouped by day
  // NOTE: the key is the day and the value is an array of feedback data
  const grouped_data: Map<number, any> = new Map();
  jdata.forEach((elem: any) => {
    const { created_at, data } = elem;
    const day = getDate(fromUnixTime(created_at));
    const { feedback_data } = JSON.parse(data);
    if (grouped_data.has(day)) {
      // NOTE: i hate ...spread
      grouped_data.set(day, [...grouped_data.get(day), feedback_data]);
    } else {
      grouped_data.set(day, [feedback_data]);
    }
  });

  const x_labels = [...Array(number_days).keys()].map((v) =>
    (v + 1).toString(),
  );

  // TODO: log day 1 data
  console.log('DAY 1 DATA', grouped_data.get(1));
  console.log('DAY 9 DATA', grouped_data.get(9));

  // TODO: get the average of the feedback data for each day
  grouped_data.forEach((data, day) => {
    const keys = Object.keys(data[0]);
    const total = data.length;
    // TODO: compute the average for each feedback category
    const avg = keys.map((key) => {
      const sum = data.reduce((acc: number, elem: any) => {
        return acc + elem[key];
      }, 0);
      return [key, sum / total];
    });
    if (day == 1 || day == 10) {
      console.log('AVERAGE', JSON.stringify(avg), 'DAY', day);
    }
    avg.forEach((elem) => {
      const [key, val] = elem;
      // TODO: check if the dataset already contains the key
      // if it does, update the data field
      const pos_idx = dataset.findIndex((elem: any) => elem.label === key);
      if (pos_idx !== -1) {
        dataset[pos_idx].data[day - 1] = val;
        return;
      } else {
        dataset.push(
          generate_dataset('daily', key as string, day - 1, val as number),
        );
      }
    });
  });
  console.log('DATASET DAILY ->', dataset);
  // TODO: make the keys pretty
  return {
    labels: x_labels,
    datasets: pretty_dataset(dataset),
  };
};

const generate_monthly_report = (data: string): ChartData => {
  const dataset: any[] = [];
  const jdata = JSON.parse(data);
  // NOTE: this map contains the feedback data grouped by month
  // NOTE: the key is the month and the value is an array of feedback data
  const grouped_data: Map<number, any> = new Map();
  jdata.forEach((elem: any) => {
    const { created_at, data } = elem;
    const month = getMonth(fromUnixTime(created_at));
    const { feedback_data } = JSON.parse(data);
    if (grouped_data.has(month)) {
      // NOTE: i hate ...spread
      grouped_data.set(month, [...grouped_data.get(month), feedback_data]);
    } else {
      grouped_data.set(month, [feedback_data]);
    }
  });

  // NOTE: the idx of the month corresponds to the
  // idx of the elem in the data field of the dataset object
  const x_labels = [
    'January',
    'February',
    'March',
    'April',
    'May',
    'June',
    'July',
    'August',
    'September',
    'October',
    'November',
    'December',
  ];

  // TODO: get the average of the feedback data for each month
  grouped_data.forEach((data, month) => {
    const keys = Object.keys(data[0]);
    const total = data.length;
    // TODO: compute the average for each feedback category
    const avg = keys.map((key) => {
      const sum = data.reduce((acc: number, elem: any) => {
        return acc + elem[key];
      }, 0);
      return [key, sum / total];
    });
    avg.forEach((elem) => {
      const [key, val] = elem;
      // TODO: check if the dataset already contains the key
      // if it does, update the data field
      const pos_idx = dataset.findIndex((elem: any) => elem.label === key);
      if (pos_idx !== -1) {
        dataset[pos_idx].data[month] = val;
        return;
      } else {
        dataset.push(
          generate_dataset('monthly', key as string, month, val as number),
        );
      }
    });
  });
  return {
    labels: x_labels,
    datasets: pretty_dataset(dataset),
  };
};

const generate_report = (report_type: ReportType, data: string): ChartData => {
  switch (report_type) {
    case 'daily':
      return generate_daily_report(data);
    case 'weekly':
      return generate_weekly_report(data);
    case 'monthly':
      return generate_monthly_report(data);
    default:
      throw new Error('Invalid report type');
  }
};

export { generate_report };

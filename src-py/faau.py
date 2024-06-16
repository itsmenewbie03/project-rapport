import json
import os
import threading
import time
from datetime import datetime

import cv2
import pandas as pd
from deepface import DeepFace

is_active = False
frame_res = []
start = None


def analyze_frame(frame, debug=False):
    try:
        res = DeepFace.analyze(frame, actions=["emotion", "gender", "age"])
        if debug:
            dominant_emotion = res[0]["dominant_emotion"]
            dominant_gender = res[0]["dominant_gender"]
            age = res[0]["age"]
            debug_image = cv2.putText(
                frame,
                f"EMOTION: {dominant_emotion} | GENDER: {dominant_gender} | AGE: {age}",
                (20, 20),
                cv2.FONT_HERSHEY_SIMPLEX,
                0.5,
                (255, 0, 0),
                1,
                cv2.LINE_AA,
            )
            cv2.imwrite(f"image_{datetime.now()}.png", debug_image)
        return res
    except Exception as e:
        if debug:
            print(f"[PYTHON]: Error during analysis\nERROR: {e}")
            cv2.imwrite(f"failed_{datetime.now()}.png", frame)
        raise Exception("Failed to detect face")


class BackgroundAnalysis:
    def __init__(self, feedback_id):
        self.feedback_id = feedback_id
        self.cap = None  # Initialize capture object lazily
        self.thread = None  # Initialize thread object lazily
        self.running = False  # Flag to track ongoing analysis

    def start_analysis(self):
        global is_active
        global start
        if not self.running:
            # NOTE: we intentionally introduce a delay
            # so that ffmpeg can exit gracefully
            time.sleep(1)
            self.cap = cv2.VideoCapture(f"{self.feedback_id}.mp4")
            self.running = True
            self.thread = threading.Thread(target=self._run_analysis)
            self.thread.daemon = True  # Ensure thread doesn't block program exit
            self.thread.start()
            is_active = True
            start = time.time()

    def _run_analysis(self):
        global is_active
        counter = 0
        while self.running:
            ret, frame = self.cap.read()  # pyright: ignore
            if not ret:
                print(
                    f"[PYTHON]: Unable to capture frame from file {self.feedback_id}.mp4"
                )
                self.running = False
                break
            if counter % 6 == 0:
                print(f"[PYTHON]: Analyzing frame {counter}")
                try:
                    res = analyze_frame(frame)
                except:
                    counter += 1
                    continue
                frame_res.append(res[0])
                print(res)
                counter += 1
                continue
            counter += 1
            print(f"[PYTHON]: Skipping frame {counter}")
        self.cap.release()  # pyright: ignore
        cv2.destroyAllWindows()
        is_active = False
        end = time.time()
        print(f"[PYTHON]: Time taken: {end - start} seconds")  # pyright: ignore
        print("[PYTHON]: Analysis thread stopped")
        print(f"[PYTHON]: Total frames analyzed: {len(frame_res)}")
        open(f"{self.feedback_id}_frame_data.json", "w").write(json.dumps(frame_res))
        aggregate(self.feedback_id)
        frame_res.clear()

    def stop_analysis(self):
        self.running = False
        if self.thread:
            self.thread.join()  # Wait for thread to finish gracefully


def start_analysis(feedback_id):
    global is_active
    if is_active:
        return False
    analyzer = BackgroundAnalysis(feedback_id)
    analyzer.start_analysis()
    return True


def aggregate(feedback_id: str):
    fname = f"{feedback_id}_feedback.json"
    if not os.path.isfile(fname):
        print("Must be in src-tauri")
        # NOTE: this is to handle the case during testing
        # where the feedback data is being written in the
        # src-tauri directory
        fname = f"src-tauri/{feedback_id}_feedback.json"
    feedback_data_df = json.loads(open(fname, "r").read())
    frame_data_df = pd.read_json(f"{feedback_id}_frame_data.json")
    total_entries = len(frame_data_df)

    # INFO: emotion aggregation
    dominant_emotion_counts = frame_data_df["dominant_emotion"].value_counts()
    dominant_emotion_percentages = (
        dominant_emotion_counts.div(total_entries) * 100
    ).round(2)
    dominant_emotion = dominant_emotion_percentages.idxmax()
    dominant_emotion_percent = dominant_emotion_percentages.max()

    # INFO: emotion aggregation
    dominant_gender_counts = frame_data_df["dominant_gender"].value_counts()
    dominant_gender_percentages = (
        dominant_gender_counts.div(total_entries) * 100
    ).round(2)
    dominant_gender = dominant_gender_percentages.idxmax()
    dominant_gender_percent = dominant_gender_percentages.max()
    mean_age = frame_data_df["age"].mean().round(2)  # pyright: ignore

    # TODO: merge the data
    feedback_data_df["dominant_emotion"] = dominant_emotion
    feedback_data_df["dominant_emotions"] = dominant_emotion_percentages.to_dict()
    feedback_data_df["dominant_emotion_percent"] = dominant_emotion_percent
    feedback_data_df["dominant_gender"] = dominant_gender
    feedback_data_df["dominant_gender_percent"] = dominant_gender_percent
    feedback_data_df["mean_age"] = mean_age

    print(f"Dominant Emotion: {dominant_emotion} ({dominant_emotion_percent}%)")
    print(f"Dominant Gender: {dominant_gender} ({dominant_gender_percent}%)")
    print(f"Age: {mean_age}")
    open(f"{feedback_id}_aggregate.json", "w").write(json.dumps(feedback_data_df))


def preload_model():
    """
    Preload to avoid latency during first call
    """
    DeepFace.analyze(
        "Blank.jpg", actions=["emotion", "gender", "age"], enforce_detection=False
    )

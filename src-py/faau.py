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
analyzer = None


def analyze_frame(frame, debug=False):
    try:
        res = DeepFace.analyze(frame, actions=["emotion"])
        if debug:
            dominant_emotion = res[0]["dominant_emotion"]
            debug_image = cv2.putText(
                frame,
                f"EMOTION: {dominant_emotion}",
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
    def __init__(self, feedback_id, debug=False):
        self.feedback_id = feedback_id
        self.cap = None  # Initialize capture object lazily
        self.thread = None  # Initialize thread object lazily
        self.running = False  # Flag to track ongoing analysis
        self.debug = debug
        self.capture_now = False

    def start_analysis(self):
        global is_active
        global start
        if not self.running:
            self.cap = cv2.VideoCapture(0)
            self.running = True
            self.thread = threading.Thread(target=self._run_analysis)
            self.thread.daemon = True  # Ensure thread doesn't block program exit
            self.thread.start()
            is_active = True
            start = time.time()

    def _run_analysis(self):
        global is_active
        while self.running:
            ret, frame = self.cap.read()  # pyright: ignore
            if not self.capture_now:
                continue
            if not ret:
                print(
                    f"[PYTHON]: Unable to capture frame from file {self.feedback_id}.mp4"
                )
                self.running = False
                break

            print(
                f"[PYTHON]: Analyzing frame because self.capture_now is {self.capture_now}"
            )
            try:
                res = analyze_frame(frame, self.debug)
            except:
                print("[PYTHON]: Failed to detect face in _run_analysis")
                continue
            frame_res.append(res[0])
            print(res)
            self.capture_now = False
        self.cap.release()  # pyright: ignore
        cv2.destroyAllWindows()
        is_active = False
        end = time.time()
        print(f"[PYTHON]: Time taken: {end - start} seconds")  # pyright: ignore
        print("[PYTHON]: Analysis thread stopped")
        print(f"[PYTHON]: Total frames analyzed: {len(frame_res)}")
        frame_res.clear()

    def capture_frame(self):
        if not self.running:
            return False
        self.capture_now = True
        time.sleep(1)  # Wait for analysis to complete
        # INFO: we will return the last frame result
        return frame_res[-1]

    def stop_analysis(self):
        self.running = False
        if self.thread:
            self.thread.join()  # Wait for thread to finish gracefully


def aggregate(feedback_id: str, debug=False):
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

    # TODO: merge the data
    feedback_data_df["dominant_emotion"] = dominant_emotion
    feedback_data_df["dominant_emotions"] = dominant_emotion_percentages.to_dict()
    feedback_data_df["dominant_emotion_percent"] = dominant_emotion_percent

    print(f"Dominant Emotion: {dominant_emotion} ({dominant_emotion_percent}%)")
    open(f"{feedback_id}_aggregate.json", "w").write(json.dumps(feedback_data_df))
    if not debug:
        # INFO: we will clean the files if debug is false
        os.remove(fname)  # Feedback Data
        os.remove(f"{feedback_id}.mp4")  # Video
        os.remove(f"{feedback_id}_frame_data.json")  # Frame Data


def preload_model():
    """
    Preload to avoid latency during first call
    """
    DeepFace.analyze("Blank.jpg", actions=["emotion"], enforce_detection=False)


def poll_analysis(feedback_id, debug):
    print("[PYTHON]: We got polled for analysis")
    fname = f"{feedback_id}_aggregate.json"
    if not os.path.isfile(fname):
        return False
    data = open(fname, "r").read()
    if not debug:
        os.remove(fname)
    return data


def start_analysis(feedback_id, debug):
    print(
        f"[PYTHON]: Starting analysis for {feedback_id} with debug: {debug} {type(debug)}"
    )
    global is_active
    global analyzer
    if is_active:
        return False
    analyzer = BackgroundAnalysis(feedback_id, debug)
    analyzer.start_analysis()
    return True


def stop_analysis():
    global analyzer
    global is_active
    if not analyzer or not is_active:
        return False
    analyzer.stop_analysis()
    analyzer = None
    is_active = False
    return True


def take_photo(feedback_id, quality):
    print(f"[PYTHON]: Taking photo for {feedback_id} for quality: {quality}")
    global analyzer
    if not analyzer:
        return False
    return analyzer.capture_frame()

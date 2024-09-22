import base64
import os
import threading
import time

import cv2

is_active = False
frame_res = []
photos = []
start = None
analyzer = None


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
            ret, frame = self.cap.read()
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
                # res = analyze_frame(frame, self.debug)
                # NOTE: we need to introduce mitigations for posibilities
                # where there is no face in the photo, the current implementation
                # has a retry feature but we can't have that are we're going to extract
                # the faau from the main api
                photos.append(frame)
            except:
                print("[PYTHON]: Failed to detect face in _run_analysis")
                continue
            # NOTE: we only take the first face
            # frame_res.append(res[0])
            # print(res)
            self.capture_now = False
        self.cap.release()
        cv2.destroyAllWindows()
        is_active = False
        end = time.time()
        print(f"[PYTHON]: Time taken: {end - start} seconds")  # pyright: ignore
        print("[PYTHON]: Analysis thread stopped")
        photos.clear()

    def capture_frame(self):
        if not self.running:
            return False
        self.capture_now = True
        time.sleep(1)  # Wait for analysis to complete
        # INFO: we will return the last photo
        return photos[-1]

    def stop_analysis(self):
        self.running = False
        if self.thread:
            self.thread.join()  # Wait for thread to finish gracefully


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
    frame = analyzer.capture_frame()
    if isinstance(frame, bool):
        return False
    b64buf = cv2.imencode(".jpg", frame)[1]
    return base64.b64encode(b64buf)

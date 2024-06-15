import os
import re
import subprocess
import threading
import time

recording_active = False


def is_dangerous_filename(filename):
    pattern = r"[\\/:<>?*]"  # Matches various potentially dangerous characters
    return bool(re.search(pattern, filename))


def start_capture(output_file: str, fps=25, resolution="640x480"):

    global recording_active
    if recording_active:
        print("[PYTHON]: only one recording is allowed at a time!")
        return False
    recording_active = True

    def record_video():
        global recording_active
        command = [
            "ffmpeg",
            "-f",
            "v4l2",
            "-s",
            resolution,
            "-framerate",
            str(fps),
            "-i",
            "/dev/video0",
            output_file,
        ]
        try:
            process = subprocess.Popen(command)
            while recording_active:
                process.poll()
                if process.returncode is not None:

                    recording_active = False
                    break
                time.sleep(1)
            process.terminate()
            process.wait()
        except Exception as e:
            print(f"Error during recording: {e}")
        finally:
            recording_active = False

    recording_thread = threading.Thread(target=record_video)
    recording_thread.start()
    return True


def clear_capture(output_file: str):
    print(f"[PYTHON]: deleting {output_file}")
    global recording_active
    if not recording_active:
        print("[PYTHON]: no recording to clear")
        return False
    # INFO: stop the recording
    recording_active = False
    # WARN: we will delete a file with a user defined input
    if not output_file.endswith(".mp4"):
        print("[PYTHON]: attempted to delete a non mp4 file")
        return False
    if is_dangerous_filename(output_file):
        print("[PYTHON]: dangerous filename detected")
        return False
    os.remove(output_file)
    return True


def stop_capture():
    global recording_active
    if not recording_active:
        print("[PYTHON]: no recording to stop")
        return False
    recording_active = False
    return True

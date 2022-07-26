from concurrent.futures import thread
from flask import Flask
from movie_retrievals import run_user_rating_consumer, fit_and_evaluate, run_movie_consumer
import threading

app = Flask(__name__)

@app.route("/")
def index() -> str:
    return "Hello World"

if __name__ == "__main__":
    app.run(debug=True)
    # Run Two Consumers
    # thread = threading.Thread(target= run_user_rating_consumer)
    # thread.start()
    run_user_rating_consumer()
    run_movie_consumer()
    # thread.join()

from app import app
from flask import render_template, redirect, url_for, request, flash

@app.route("/")
def index():
    return "home page"

@app.route("/watch")
def watch():
    # link the Drive with all the movies.
    # folder name -> {episode[i].mp4, header_image.png, bio.text}
    shows: list = ['links', 'to', 'shows']
    return render_template('watch.html', shows)



import __wbg_init, { vis, get_max_turn } from "./wasm/pkg/wasm.js";
__wbg_init().then(() => {});

const options = {
  type: "openDirectory",
};

let selectedDirHandle = null;

async function fileselect() {
  selectedDirHandle = await window.showDirectoryPicker(options);
  updateInOut();
}
window.fileselect = fileselect;

async function updateInOut() {
  const seed = document.getElementById("seed").value;
  try {
    const outfilename = `${String(seed).padStart(4, "0")}.out`;
    const fileHandle = await selectedDirHandle.getFileHandle(outfilename);
    const file = await fileHandle.getFile();
    const text = await file.text();
    output.value = text;
  } catch (err) {
    output.value = "";
  }
  try {
    const errorfilename = `${String(seed).padStart(4, "0")}.err`;
    const fileHandle = await selectedDirHandle.getFileHandle(errorfilename);
    const file = await fileHandle.getFile();
    const text = await file.text();
    errorbox.value = text;
  } catch (err) {
    errorbox.value = "";
  }

  updateResult();
}
window.updateInOut = updateInOut;

function visualize() {
  const error_file = document.getElementById("errorbox").value;
  const t = document.getElementById("turn").value;
  const seed = document.getElementById("seed").value;
  try {
    const res = vis(error_file, t, seed);
    document.getElementById("score").innerHTML = "Score = " + res.score;
    document.getElementById("result").innerHTML = res.svg;

    const info = document.getElementById("info");
    while (info.childNodes.length > 1) {
      info.removeChild(info.lastChild);
    }
    res.data.forEach((datum) => {
      const dataItem = document.createElement("div");
      dataItem.className = "data-item";
      dataItem.textContent = `${datum.name} = ${datum.value}`;
      info.appendChild(dataItem);
    });
  } catch (error) {
    document.getElementById("result").innerHTML = "<p>Invalid</p>";
  }
}
window.visualize = visualize;

function update_t(t) {
  const max_turn = Number(document.getElementById("turn").max);
  const new_turn = Math.min(Math.max(0, t), max_turn);
  document.getElementById("turn").value = new_turn;
  document.getElementById("t_bar").value = new_turn;
  visualize();
}
window.update_t = update_t;

var prev = Date.now();
const play = document.getElementById("play");
const speed = document.getElementById("speed");

function start_autoplay() {
  if (Number(document.getElementById("turn").value) >= Number(document.getElementById("turn").max)) {
    document.getElementById("turn").value = 0;
  }
  prev = Date.now();
  play.value = "■";
  update_t(document.getElementById("turn").value);
}
window.start_autoplay = start_autoplay;

function updateResult() {
  play.value = "▶";
  const error = document.getElementById("errorbox").value;
  try {
    const seed = document.getElementById("seed").value;
    const t = get_max_turn(error, seed);
    document.getElementById("turn").max = t;
    document.getElementById("t_bar").max = t;
    document.getElementById("speed").max = t / 3;
    document.getElementById("speed").value = t / 5;
    update_t(t);
  } catch (error) {
    document.getElementById("result").innerHTML = "<p>Invalid</p>" + error;
  }
}
window.updateResult = updateResult;

play.onclick = (event) => {
  if (play.value == "■") {
    play.value = "▶";
  } else {
    start_autoplay();
  }
};

function autoplay() {
  if (play.value == "■") {
    const now = Date.now();
    let s = 2000;
    if ((now - prev) * speed.value >= s) {
      const inc = Math.floor(((now - prev) * speed.value) / s);
      prev += Math.floor((inc * s) / speed.value);
      update_t(Number(document.getElementById("turn").value) + inc);
      if (Number(document.getElementById("turn").value) >= Number(document.getElementById("turn").max)) {
        play.value = "▶";
      }
    }
  }
  requestAnimationFrame(autoplay);
}
autoplay();

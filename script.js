const wasmExecTimes = [];
const jsExecTimes = [];
let wasmRunning = false;
let jsRunning = false;
let currentWasmPitch = -1;
let currentJsPitch = -1;

let wasmPitchDomEl = document.querySelector('#wasmPitch');
let jsPitchDomEl = document.querySelector('#jsPitch');
let wasmAvgExecTimeDomEl = document.querySelector('#wasmExecTime');
let jsAvgExecTimeDomEl = document.querySelector('#jsExecTime');

setInterval(function() {
  wasmPitchDomEl.innerHTML = currentWasmPitch > 0 ? currentWasmPitch.toFixed(2) + 'Hz' : '-';
  jsPitchDomEl.innerHTML = currentJsPitch > 0 ? currentJsPitch.toFixed(2) + 'Hz' : '-';

  if(wasmExecTimes.length > 0) {
    wasmAvgExecTimeDomEl.innerHTML = (wasmExecTimes.reduce((curr, acc) => {
      return acc + curr;
    }, 0) / wasmExecTimes.length).toFixed(2) + 'ms';
  }

  if(jsExecTimes.length > 0) {
    jsAvgExecTimeDomEl.innerHTML = (jsExecTimes.reduce((curr, acc) => {
      return acc + curr;
    }, 0) / jsExecTimes.length).toFixed(2) + 'ms';
  }
}, 300);

function toggleWasm(button) {
  if(wasmRunning) {
    wasmRunning = false;
    wasmExecTimes.length = 0;
    button.innerHTML = 'Start';
  } else {
    wasmRunning = true;
    jsExecTimes.length = 0;
    button.innerHTML = 'Stop';
  }
}

function toggleJs(button) {
  if(jsRunning) {
    jsRunning = false;
    button.innerHTML = 'Start';
  } else {
    alert(`Don't let it run too long, and don't be confused if it continues after stopping.\
 It needs some time to finish calculation for already recorded chunks.`);
    jsRunning = true;
    button.innerHTML = 'Stop';
  }
}

const worker = new Worker('js-tuner/worker.js');
worker.onmessage = function(e) {
  currentJsPitch = e.data.pitch;

  jsExecTimes.push(e.data.execTime);
};


Module['noExitRuntime'] = true;
Module.onRuntimeInitialized = function() {
  const bufferLength = 1024;

  navigator.mediaDevices.getUserMedia({audio: true, video: false}).then(stream => {
    const context = new AudioContext();
    const source = context.createMediaStreamSource(stream);

    const node = context.createScriptProcessor(bufferLength, 1, 1);

    // listen to the audio data, and record into the buffer
    node.onaudioprocess = function(e) {
      let data = e.inputBuffer.getChannelData(0);

      if(wasmRunning) {
        let buffer = Module._malloc(bufferLength * data.BYTES_PER_ELEMENT);

        Module.HEAPF32.set(data, buffer >> 2);

        const startTime = window.performance.now();
        currentWasmPitch = Module._get_pitch(buffer, data.length, e.inputBuffer.sampleRate);
        wasmExecTimes.push(window.performance.now() - startTime);
      }

      if(jsRunning) {
        worker.postMessage({
          buffer: data.buffer,
          sampleRate: e.inputBuffer.sampleRate
        }, [data.buffer]);
      }
    }

    source.connect(node);
    node.connect(context.destination);
  });
}

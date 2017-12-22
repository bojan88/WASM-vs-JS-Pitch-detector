Module['noExitRuntime'] = true;

Module.onRuntimeInitialized = function() {
  const pitchEl = document.querySelector('#pitch');
  const bufferLength = 1024;

  navigator.mediaDevices.getUserMedia({ audio: true, video: false }).then(stream => {
    const context = new AudioContext();
    const source = context.createMediaStreamSource(stream);

    const node = context.createScriptProcessor(bufferLength, 1, 1);

    // listen to the audio data, and record into the buffer
    node.onaudioprocess = function(e) {
      let data = e.inputBuffer.getChannelData(0);
      let buffer = Module._malloc(bufferLength * data.BYTES_PER_ELEMENT);

      Module.HEAPF32.set(data, buffer >> 2);

      let startTime = window.performance.now();
      let pitch = Module._get_pitch(buffer, data.length, e.inputBuffer.sampleRate);

      // console.log(pitch);
      // pitchEl.innerHTML = pitch;
    }

    source.connect(node);
    node.connect(context.destination);
  });
}

<template>
  <canvas id="display"></canvas>
  <input id="seedInput" v-model="stringSeed" @change="updateCanvas" placeholder="Mnemonic seed" />
  <div class="slider-container">
    <input id="sizeSlider" v-model="resolution" @change="updateCanvas" type="range" min="10" max="910" step="30" class="slider">
  </div>
  <span id="resolutionSpan">Resolution: {{resolution / 2 * 3}}x{{resolution}}</span>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import init, { generate_from_string, init_console_errors } from '@gregorykogan/mnemonic-pictures';

export default defineComponent({
  name: "App",
  data: () => ({
    wasmLoaded: false,
    stringSeed: "Seed",
    resolution: 400,
  }),
  mounted() {
      init().then(() => { 
        this.wasmLoaded = true;
        init_console_errors();
        this.updateCanvas();
      });
  },
  methods: {
    updateCanvas() {
      let canvas = document.getElementById("display") as HTMLCanvasElement;
      canvas.width = this.resolution / 2 * 3;
      canvas.height = this.resolution;
      generate_from_string("display", this.stringSeed);
    },
  }
});
</script>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>

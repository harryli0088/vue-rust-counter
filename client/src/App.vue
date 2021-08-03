<template>
  <div id="container">
    <div>
      <div>
        <img id="vue-logo" alt="Vue logo" src="./assets/logo.png">
        <img id="rust-logo" alt="Rust logo" src="./assets/rust.png">
      </div>
      <Counter msg="Vue + Rust Counter App"/>
    </div>

    <footer>Ferris the Crab image from <a href="https://rustacean.net/" target="_blank" rel="noopener">https://rustacean.net/</a></footer>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Counter from './components/Counter.vue';

export default defineComponent({
  name: 'App',
  components: {
    Counter
  }
});
</script>

<style lang="scss">
body {
  margin: 0;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#container {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
}

#vue-logo, #rust-logo {
  height: 150px;
}
@media only screen and (max-width: 500px) {
  #vue-logo, #rust-logo {
    height: 125px;
  }
}

footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: #ddd;
  padding: 0.25em;
}
</style>

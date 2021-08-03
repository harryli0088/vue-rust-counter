<template>
  <div id="container">
    <div>
      <img id="logo" alt="Vue logo" src="./assets/logo.png">
      <Counter msg="Vue + Rust Counter App"/>
    </div>
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

#logo {
  height: 150px;
}
</style>

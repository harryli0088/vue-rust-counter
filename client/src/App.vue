<template>
  <img alt="Vue logo" src="./assets/logo.png">
  <Todo msg="Welcome to this Vue.js + TypeScript Todo App"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Todo from './components/Todo.vue';

export default defineComponent({
  name: 'App',
  components: {
    Todo
  }
});
</script>

<style lang="scss">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>

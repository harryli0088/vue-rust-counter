<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <form v-on:submit="create">
      <input
        @change="value=$event.target.value"
        v-bind:value="createValue"
      />
      <button type="submit">Submit</button>
    </form>

    <ul>
      <li v-for="(todo, index) in todos" :key="index">
        {{todo}}
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

type State = {
  todos: string[],
  createValue: string,
  updateIndex: number,
  updateValue: string,
}
export default defineComponent({
  data: ():State => ({
    todos: [],
    createValue: "",
    updateIndex: -1,
    updateValue: "",
  }),
  methods: {
    create: function(e: Event) {
      e.preventDefault()
      console.log("this.createValue",this.createValue)
    },
    delete: function() {
      console.log("this.createValue",this.createValue)

    },
    update: function() {
      console.log("this.createValue",this.createValue)

    },
  },
  mounted: function() {
    fetch("http://localhost:8080/todos").then(res => res.json()).then(console.log).catch(console.error)
  },
  name: 'Todo',
  props: {
    msg: String,
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
h3 {
  margin: 40px 0 0;
}
ul {
  todos-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>

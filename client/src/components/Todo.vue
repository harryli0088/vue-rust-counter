<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <form v-on:submit="create">
      <input
        @change="value=$event.target.value"
        v-bind:value="value"
      />
      <button type="submit">Submit</button>
    </form>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

type State = {
  value: string,
}
export default defineComponent({
  data: ():State => ({
    value: "",
  }),
  methods: {
    create: function(e: Event) {
      e.preventDefault()
      console.log("this.value",this.value)
    },
    delete: function() {
      console.log("this.value",this.value)

    },
    update: function() {
      console.log("this.value",this.value)

    },
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
  list-style-type: none;
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

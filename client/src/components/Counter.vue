<template>
  <div>
    <h1>{{ msg }}</h1>
    <!-- <form v-on:submit="create">
      <input
        @change="createValue=$event.target.value"
        v-bind:value="createValue"
      />
      <button type="submit">Submit</button>
    </form>

    <hr/>

    <ul>
      <li v-for="(todo, index) in todos" :key="index">
        {{todo}}
      </li>
    </ul> -->
    <div><b>Count:</b> {{count}}</div>
    <div><button v-on:click="decrement">Decrement</button><button v-on:click="increment">Increment</button></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { SERVER_URL } from '../utils/constants'

type State = {
  count: number,
}
export default defineComponent({
  data: ():State => ({
    count: 0
  }),
  methods: {
    decrement: function(e: Event) {
      e.preventDefault()
      fetch(`${SERVER_URL}/decrement`, {method: "POST"}).then(
        res => res.text()
      ).then(this.setCount).catch(console.error)
    },
    getCount: function() {
      fetch(`${SERVER_URL}/count`).then(
        res => res.text()
      ).then(this.setCount).catch(
        console.error
      )
    },
    increment: function(e: Event) {
      e.preventDefault()
      fetch(`${SERVER_URL}/increment`, {method: "POST"}).then(
        res => res.text()
      ).then(this.setCount).catch(console.error)
    },
    setCount: function(count: string) {
      this.count = parseInt(count)
    }
  },
  mounted: function() {
    this.getCount()
  },
  name: 'Todo',
  props: {
    msg: String,
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">

</style>

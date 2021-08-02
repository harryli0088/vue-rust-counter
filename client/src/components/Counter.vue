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
    <div>{{error}}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import handleResponse from '../utils/handleResponse'
import { SERVER_URL } from '../utils/constants'

type State = {
  error: string,
  count: number,
}
export default defineComponent({
  data: ():State => ({
    error: "",
    count: 0
  }),
  methods: {
    decrement: function(e: Event) {
      e.preventDefault()
      fetch(`${SERVER_URL}/decrement`, {method: "POST"}).then(handleResponse).then(
        res => res.text()
      ).then(this.setCount).catch(this.handleError)
    },
    getCount: function() {
      fetch(`${SERVER_URL}/count`).then(handleResponse).then(
        res => res.text()
      ).then(this.setCount).catch(
        this.handleError
      )
    },
    handleError: function(err: Error) {
      this.error = err.message
    },
    increment: function(e: Event) {
      e.preventDefault()
      fetch(`${SERVER_URL}/increment`, {method: "POST"}).then(handleResponse).then(
        res => res.text()
      ).then(this.setCount).catch(this.handleError)
    },
    setCount: function(count: string) {
      this.error = ""
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

<template>
  <div>
    <h2>{{ msg }}</h2>
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
    <br/>
    
    <div v-if="status === getConnectionStatus().connected">
      <div id="count"><b>Count:</b> {{count}}</div>
      <br/>
      <div><button id="increment" v-on:click="increment">Increment ++</button></div>
      <div><button id="decrement" v-on:click="decrement">Decrement --</button></div>
    </div>
    <div v-else>
      Connecting...
    </div>

    <div>{{error}}</div>

    <br/>
    <br/>
    
    <div><a href="https://github.com/harryli0088/vue-rust-counter" target="_blank" rel="noopener noreferrer">Github</a></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import handleResponse from '../utils/handleResponse'
import { SERVER_URL } from '../utils/constants'

enum ConnectionStatus {
  mounting,
  loading,
  connected,
  error,
}

type State = {
  error: string,
  count: number,
  status: ConnectionStatus,
}
export default defineComponent({
  data: ():State => ({
    error: "",
    count: 0,
    status: ConnectionStatus.mounting,
  }),
  methods: {
    decrement: function(e: Event) {
      e.preventDefault()
      fetch(`${SERVER_URL}/decrement`, {method: "POST"}).then(handleResponse).then(
        res => res.text()
      ).then(this.setCount).catch(this.handleError)
    },
    getConnectionStatus: function () {
      return ConnectionStatus
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
      this.status = ConnectionStatus.connected
    }
  },
  mounted: function() {
    this.status = ConnectionStatus.loading
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
  #count {
    font-size: 30px;  
  }

  button {
    padding: 0.5em 1em;
    margin-bottom: 0.5em;
    cursor: pointer;
    background-color: gray;
    border-radius: 5px;
    color: white;
    font-weight: bold;
    border: none;
    border-bottom: 2px solid #666;

    &:active {
      border-bottom-width: 0;
      margin-top: 2px;
    }
  }

  #increment {
    background-color: #1ABC9C;
  }

  #decrement {
    background-color: #EC7063;
  }
</style>

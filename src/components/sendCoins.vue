<script>
const invoke = window.__TAURI__.invoke

export default {
  data() {
    return {
      send_result: ""
    }
  },

  methods: {
    send_coins(value, to) {
      invoke('send_coins', {value: value, to:to}).then((res) => this.send_result = res);
    }
  }
}
</script>


<template>
<input type="text" name="address" id="addressInput" ref="addressInput" placeholder="Address">
<input type="number" name="amount" id="amountInput" ref="amountInput" placeholder="Amount">

<button @click="send_coins(this.$refs.amountInput.value, this.$refs.addressInput.value)">Send</button>
<p> {{ send_result }} </p>


</template>
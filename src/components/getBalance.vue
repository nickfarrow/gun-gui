<script>
const invoke = window.__TAURI__.invoke

export default {
  data() {
    return {
      balance: {}
    }
  },

  methods: {
    getBalance() {
      invoke('get_balance').then((bal) => this.balance = JSON.parse(bal));
    }
  }
}
</script>


<template>
<ul style="list-style-type: none;">
  <li>Available: {{ balance['available'] }}</li>
  <li>Confirmed: {{ balance['confirmed'] }}</li>
  <li>In-Use: {{ balance['in-use'] }}</li>
  <li>Locked: {{ balance['locked'] }}</li>
  <li>Unclaimed: {{ balance['unclaimed'] }}</li>
  <li>Unconfirmed: {{ balance['unconfirmed'] }}</li>
</ul>
<button @click="getBalance">Get Balance</button>

</template>
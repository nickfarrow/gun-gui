<script>
import getBalance from './components/getBalance.vue'
import sendCoins from './components/sendCoins.vue'

export default {
  components: {
    getBalance,
    sendCoins
  }
}
</script>

<style>
html, body {
    background-color:rgb(30, 80, 100);
}
</style>

<template>
  <h1>gun-gui</h1>
  <getBalance />
  <br>
  <br>
  <sendCoins />
</template>
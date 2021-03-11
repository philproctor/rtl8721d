<template>
  <div class="content">
    <h1>Overview</h1>
    <h2>Services online: {{ online }}</h2>
    <h3><button v-on:click="getStatus">update</button></h3>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
export default {
  name: 'Overview',
  methods: {
    ...mapActions(['getStatus']),
  },
  computed: {
    ...mapGetters({ online: 'serviceOnline' }),
  },
  mounted: function() {
    this.getStatus()
  },
}
</script>

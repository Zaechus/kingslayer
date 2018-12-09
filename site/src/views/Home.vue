<template>
  <div class="home">
    <Description msg="Kingslayer"/>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import Description from '@/components/Description.vue'; // @ is an alias to /src

export default Vue.extend({
  name: 'home',
  components: {
    Description,
  },
});
</script>

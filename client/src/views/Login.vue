<template>
  <div class="login">
    <h1>This is an login page</h1>
    <button @click="test">Test</button>
  </div>
</template>


<script lang="ts">
import {Vue, Options} from "vue-class-component";

@Options({})
export default class Login extends Vue {
  public test() {
    console.log("test");
  }
}
</script>

<style scoped>

</style>
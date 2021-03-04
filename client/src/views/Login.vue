<template>
  <div>
    <form @submit="login">
      <label><b>User Name
      </b>
      </label>
      <input type="text" name="Uname" id="Uname" placeholder="Username" v-model="username">
      <br><br>
      <label><b>Password
      </b>
      </label>
      <input type="Password" name="Pass" id="Pass" placeholder="Password" v-model="password">
      <br><br>
      <input type="button" name="log" id="log" value="Log In Here" @click="login">
    </form>
  </div>
</template>

<script lang="ts">
import { Vue } from 'vue-class-component';

export default class Login extends Vue {
  username: String = ''

  password: String = ''

  public async login() {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id: this.username, password: this.password }),
    };
    fetch('/auth/login', requestOptions)
      .then((response) => response.json())
      .then((data) => console.log(data));
  }
}
</script>

<style scoped>

</style>

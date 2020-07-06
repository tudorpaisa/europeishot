<template>
  <div id="app">
    <el-menu :router="true" :default-active="activeLink" mode="horizontal">
      <el-menu-item index="/">Home</el-menu-item>
      <el-menu-item index="/info">Info</el-menu-item>
      <el-menu-item
        ><a href="https://github.com/tudorpaisa/europeishot"
          >GitHub</a
        ></el-menu-item
      >
    </el-menu>
    <router-view></router-view>
  </div>
</template>

<script>
export default {
  name: "App",
  components: {},
  data() {
    return {
      activeLink: null
    };
  },
  mounted() {
    this.activeLink = this.$route.path;
  },
  watch: {
    $route(to) {
      this.activeLink = to.path;
    }
  }
};
</script>

<style lang="scss">
@import url("https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,400;0,600;1,400&family=Yellowtail&display=swap");

h1 {
  font-family: "Yellowtail", sans-serif;
  text-align: center;
  font-size: 3em;
}

#app {
  font-family: "Open Sans", sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>

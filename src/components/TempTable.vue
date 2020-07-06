<template>
  <el-row justify="center">
    <el-col
      :xs="{ span: 20, offset: 2 }"
      :sm="{ span: 18, offset: 3 }"
      :md="{ span: 12, offset: 6 }"
      :lg="{ span: 12, offset: 6 }"
      :xl="{ span: 8, offset: 8 }"
    >
      <h1>Europe is Hot</h1>

      <div v-html="msg"></div>
      <el-divider />

      <el-table :data="cityTemps">
        <el-table-column prop="cityName" label="City" />
        <el-table-column prop="cityTemp" label="Temperature" width="160" />
      </el-table>
    </el-col>
  </el-row>
</template>

<script>
import axios from "axios";
import showdown from "showdown";

export default {
  name: "TempTable",
  components: {},
  data() {
    return {
      mdText: `Over the past couple of years, Europe experienced a series of [record-breaking heat waves](https://en.wikipedia.org/wiki/July_2019_European_heat_wave).

In the event of [yet](https://edition.cnn.com/2020/06/25/weather/uk-europe-heat-wave-forecast/index.html) [another](https://www.forbes.com/sites/emanuelabarbiroglio/2020/05/15/europe-should-prepare-now-for-hot-2020-summer/) scorching summer, this website will tell you just how (un)lucky you are compared to the rest of Europe.`,
      msg: null,
      cityTemps: []
    };
  },
  mounted() {
    const converter = new showdown.Converter();
    this.msg = converter.makeHtml(this.mdText);
    axios
      .get("http://localhost:5000/temps/", { crossdomain: true })
      .then(response => {
        response.data.temps.forEach(element =>
          this.cityTemps.push({
            cityName: element.name,
            cityTemp: element.main.temp
          })
        );
      })
      .catch(error => console.log(error));
  }
};
</script>
<style lang="scss"></style>

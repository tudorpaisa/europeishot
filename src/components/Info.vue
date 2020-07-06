<template>
  <el-row>
    <el-col
      :xs="{ span: 20, offset: 2 }"
      :sm="{ span: 18, offset: 3 }"
      :md="{ span: 12, offset: 6 }"
      :lg="{ span: 12, offset: 6 }"
      :xl="{ span: 8, offset: 8 }"
    >
      <h1>Info</h1>
      <div v-html="msg"></div>
    </el-col>
  </el-row>
</template>

<script>
import showdown from "showdown";

export default {
  name: "Info",
  components: {},
  data() {
    return {
      mdText: `_Europe is Hot_ is supposed to poke fun at a [_very serious_ problem](https://en.wikipedia.org/wiki/Global_warming). Had it not been for human-induced climate change, these events would have been [much milder](https://www.worldweatherattribution.org/human-contribution-to-the-record-breaking-july-2019-heat-wave-in-western-europe/).

## Frequently (un)asked questions

### Why even make this website?

Over the past few years I've had a morbid fascination to compare the temperatures of the Netherlands (where I currently live) with that of other European cities (and cry). Since opening 10-20 tabs of Accuweather became quite tiresome very quickly, I set out to automate this entire process. This website is my most recent take on the project.

### Which cities are you searching for?

The 10 most populated cities per European country. I get my list from this website:

[https://worldpopulationreview.com/continents/cities/europe](https://worldpopulationreview.com/continents/cities/europe)

### What can I do stop it?

> **FOREWARNING**: I am not a meteorological or climate change expert, so take my advice with a grain of salt.

You can [move to the Canadian Northwestern Territories](https://www.zillow.com/nt/) or just do the right thing.

Probably the most impactful thing you can do is to speak-up to your elected officials and ensure that they enact laws which limit carbon emissions. Otherwise you can start from [here](https://lmgtfy.com/?q=how+to+stop+climate+change&pp=1&s=d).
`,
      msg: null
    };
  },
  mounted() {
    const converter = new showdown.Converter();
    this.msg = converter.makeHtml(this.mdText);
  }
};
</script>

<template>
  <section class="title container">
    <section class="rings">
      <div class="ring his" />
      <div class="ring hers" />
    </section>

    <h1>
      <span class="underline">Blaine</span>
      <span class="body-type"> is getting married to </span>
      <span class="underline">Ling Ling</span>
    </h1>

    <Card>
      <p class="date">July 6<sup>th</sup>, 2019</p>
      <p>Estes Park, CO</p>
    </Card>
  </section>
</template>

<script>
import Card from './../components/molecules/card.vue'

export default {
  components: {
    Card
  }
}
</script>

<style>
.title {
  padding-top: 1em;
  font-size: 5em;
  line-height: 1.25em;
  color: var(--white);
}

.rings {
  display: flex;
  justify-content: center;
  margin-bottom: 0.5em;
}

.ring {
  width: 1em;
  height: 1em;
  border: 0.1em solid var(--darkslate);
  box-shadow: inset 0 0 0em 0.03em var(--white);
  border-radius: 100%;
}

.ring.his {
  margin-right: -0.25em;
  margin-bottom: -0.25em;
}

.ring.hers {
  margin-left: -0.25em;
  margin-top: -0.25em;
}

.underline {
  text-decoration: underline;
}

.body-type {
  font-family: 'Josefin Sans', sans-serif;
}

.date {
  display: block;
  font-size: 1.25em;
  font-family: 'Abril Fatface', Georgia, 'Times New Roman', Times, serif;
  margin-bottom: 0.85em;
}

sup {
  font-size: 0.5em;
}

.title h1 {
  font-size: 0.85em;
  line-height: 1em;
  margin-bottom: 1em;
}

.title .card {
  font-size: 0.25em;
  line-height: 1em;
  max-width: 14em;
}
</style>

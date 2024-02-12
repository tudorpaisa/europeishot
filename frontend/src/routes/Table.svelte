<script>
    import { onMount } from 'svelte';

    let latestData = [];
    onMount(async () => {
        let res = await fetch("http://localhost:8080/latest");
        latestData = await res.json();
    });
</script>

<div class="">
    <div>
        <table class="weather_table">
            <thead class="weather_table_head">
                <tr>
                    <th>City</th>
                    <th>Country</th>
                    <th>Temperature</th>
                </tr>
            </thead>
            <tbody class="weather_table_body">
                {#each latestData as data}
                    <tr>
                        <td>{data.city_name}</td>
                        <td>{data.country_name}</td>
                        <td>{data.temperature_c} °C</td>
                    </tr>
                {:else}
                    <p>Loading table...</p>
                {/each}
            </tbody>
        </table>
    </div>

</div>

<style>

table {
  table-layout: fixed;
  width: 85%;
  border-collapse: collapse;
  margin-left: auto;
  margin-right: auto;
}

thead th:nth-child(1) {
  width: 30%;
}

thead th:nth-child(2) {
  width: 40%;
}

thead th:nth-child(3) {
  width: 30%;
}

tbody td:nth-child(3) {
  text-align: right;
}

td + td,
th + th { border-left: 1px solid; }
tr + tr { border-top: 1px solid; }
th { border-bottom: 1px solid; }

th,
td {
  text-align: left;
  padding: 20px;
}

tbody>tr:hover {
    background-color: var(--color-theme-3);
    color: var(--color-theme-1);
    transition: color 0.1s linear;
}
</style>

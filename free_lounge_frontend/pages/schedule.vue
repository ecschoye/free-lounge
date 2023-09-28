<template>
  <div class="container">
    <h1>Welcome to the Airport!</h1>
    <div class="table-wrapper">
      <table class="flight-schedule">
        <thead>
        <tr>
          <th>FlightNumber</th>
          <th>Departure</th>
          <th>Arrival</th>
          <th>Departure Time</th>
          <th>Arrival Time</th>
          <th>Status</th>
        </tr>
        </thead>
        <tbody>
        <tr v-for="flight in flights" :key="flight.TicketID">
          <td>{{ flight.FlightNumber }}</td>
          <td>{{ flight.DepartureAirport }}</td>
          <td>{{ flight.ArrivalAirport }}</td>
          <td>{{ flight.DepartureTime }}</td>
          <td>{{ flight.ArrivalTime }}</td>
          <td>{{ flight.Status }}</td>
        </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      flights: [
        {
          TicketID: '921784',
          FlightNumber: 'SK3766',
          DepartureAirport: 'OSL',
          ArrivalAirport: 'TRD',
          DepartureTime: '2023-10-01T15:30:00',
          ArrivalTime: '2023-10-01T16:45:00',
          Status: 'Confirmed',
        },
        {
          TicketID: '692312',
          FlightNumber: 'SK1201',
          DepartureAirport: 'OSL',
          ArrivalAirport: 'BGO',
          DepartureTime: '2023-10-01T16:30:00',
          ArrivalTime: '2023-10-01T17:45:00',
          Status: 'Confirmed',
        },
        {
          TicketID: '987210',
          FlightNumber: 'SK0932',
          DepartureAirport: 'OSL',
          ArrivalAirport: 'TRD',
          DepartureTime: '2023-10-01T16:45:00',
          ArrivalTime: '2023-10-01T18:00:00',
          Status: 'Confirmed',
        },
        {
          TicketID: '435762',
          FlightNumber: 'SK2138',
          DepartureAirport: 'OSL',
          ArrivalAirport: 'TRD',
          DepartureTime: '2023-10-01T22:30:00',
          ArrivalTime: '2023-10-01T23:45:00',
          Status: 'Confirmed',
        },
        {
          TicketID: '123456',
          FlightNumber: 'SK3932',
          DepartureAirport: 'OSL',
          ArrivalAirport: 'CPH',
          DepartureTime: '2023-10-02T10:00:00',
          ArrivalTime: '2023-10-02T11:15:00',
          Status: 'Cancelled',
        },
        {
          TicketID: '654321',
          FlightNumber: 'SK2178',
          DepartureAirport: 'OSL',
          ArrivalAirport: 'SVG',
          DepartureTime: '2023-10-02T12:00:00',
          ArrivalTime: '2023-10-02T13:30:00',
          Status: '------------------',
        }
      ]

    };
  },
};
</script>

<style scoped>
.container {
  text-align: center;
}

.table-wrapper {
  margin: auto;
}

.flight-schedule {
  width: 80%;
  border-collapse: collapse;
  margin: auto;
}

th, td {
  border: 1px solid black;
  padding: 8px;
}
</style>

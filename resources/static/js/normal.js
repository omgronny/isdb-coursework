
var JedidataApi = Vue.resource('/data{/id}');
Vue.component('jedi-data-form', {
    props: ['jedi_data'], // , 'messageAttr'
    data: function() {
        return {
            jedi_data_name: '',
            jedi_data_planet: '',
            jedi_data_team: '',
            jedi_data_ship: '',
            jedi_price: '',
        }
    },
    template:
        '<div style="max-width: 500px; margin-left: 60%">' +
        '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Name</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write name of the jedi" id="jedi_data_name" v-model="jedi_data_name" />' +
        // '</div>' +
        // '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Planet</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write name of the planet" id="jedi_data_planet" v-model="jedi_data_planet" />' +
        // '</div>' +
        // '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Team</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write size of team" id="jedi_data_team" v-model="jedi_data_team" />' +
        // '</div>' +
        // '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Ship</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write power of ship" id="jedi_data_ship" v-model="jedi_data_ship" />' +
        // '</div>' +
        // '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Price</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write price" id="jedi_price" v-model="jedi_price" />' +
        '</div>' +
        '<input type="button" value="Save" @click="save" />' +
        '</div>',
    methods: {
        save: function() {

            this.jedi_data_name = document.getElementById("jedi_data_name").value;
            this.jedi_data_planet = document.getElementById("jedi_data_planet").value;
            this.jedi_data_team = document.getElementById("jedi_data_team").value;
            this.jedi_data_ship = document.getElementById("jedi_data_ship").value;
            this.jedi_price = document.getElementById("jedi_price").value;

            var message = {
                name: this.jedi_data_name,
                planet: this.jedi_data_planet,
                team: parseInt(this.jedi_data_team),
                ship: parseInt(this.jedi_data_ship),
                normal_id: 2,
                price: parseInt(this.jedi_price)
            };

            JedidataApi.save(message).then(result =>
                result.json().then(msg => {
                    this.jedi_data.push([msg.name, msg.planet, msg.team, msg.ship]);
                    this.jedi_data_name = '';
                    this.jedi_data_planet = '';
                    this.jedi_data_team = '';
                    this.jedi_data_ship = '';
                    this.jedi_price = '';
                })
            )
        }
    }
});

Vue.component('jedi-data-list', {
    props: ['jedi_data'],
    template:
        '<div>' +
            '<jedi-data-form :jedi_data="jedi_data"/>' +
            '<table style="margin-top: -23%">' +
                '<tr><th colspan="5" width="70%">Jedi Data</th></tr>' +
                '<tr class="timer"><td>Jedi</td><td> Planet </td><td> Team-Size </td><td> Ship-Power </td><td> Price </td></tr>' +
                '<tr v-for="message in jedi_data">' +
                    '<td><i>{{ message[0] }}</i></td><td> {{ message[1] }} </td><td>{{ message[2] }}</td><td>{{ message[3] }}</td><td>{{ message[5] }}</td>' +
                '</tr>' +
            '</table>' +
        '</div>',
    created: function () {
        JedidataApi.get().then(result =>
            result.json().then(data =>
                data.forEach(msg => this.jedi_data.push(msg))
            )
        )
    }
});

var app = new Vue({
    el: '#app',
    template:
        '<div>' +
        '<div class=".overlay">' +
        '<jedi-data-list :jedi_data="jedi_data" />' +
        '<jedi-requests-list :requests="requests" />' +
        '<jedi-list :jedi="jedi" />' +
        '</div>' +
        '<div class="sand"></div>\n' +
        '<div class="bb8">\n' +
        '  <div class="antennas">\n' +
        '    <div class="antenna short"></div>\n' +
        '    <div class="antenna long"></div>\n' +
        '  </div>\n' +
        '  <div class="head">\n' +
        '    <div class="stripe one"></div>\n' +
        '    <div class="stripe two"></div>\n' +
        '    <div class="eyes">\n' +
        '      <div class="eye one"></div>\n' +
        '      <div class="eye two"></div>\n' +
        '    </div>\n' +
        '    <div class="stripe three"></div>\n' +
        '  </div>\n' +
        '  <div class="ball">\n' +
        '    <div class="lines one"></div>\n' +
        '    <div class="lines two"></div>\n' +
        '    <div class="ring one"></div>\n' +
        '    <div class="ring two"></div>\n' +
        '    <div class="ring three"></div>\n' +
        '  </div>\n' +
        '  <div class="shadow"></div>\n' +
        '</div>\n' +
        '</div>'
    ,
    data: {
        jedi_data: [],
        requests: [],
        jedi: []
    }
});

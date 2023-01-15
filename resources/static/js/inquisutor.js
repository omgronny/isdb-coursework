
var amountApi = Vue.resource('/inquisitor-id{/id}');
Vue.component('inquisitor-money', {
    data: function() {
        return {
            id: '',
            in_money: ''
        }
    },
    template:
        '<div>' +
        '<table style="margin-top: -1%">' +
        '<tr><th>My Money</th></tr>' +
        '<tr><td>{{ in_money }}</td></tr>' +
        '</table>' +
        '</div>',
    created: function () {
        this.id = 1;
        amountApi.get({ id: this.id }).then(result =>
            result.json().then(data =>
                data.forEach(amount => this.in_money = amount)
            )
        )
    }
});

var JediRequestApi = Vue.resource('/request{/id}');
Vue.component('jedi-request-form', {
    props: ['requests'], // , 'messageAttr'
    data: function() {
        return {
            jedi_request_name: '',
            jedi_request_price: '',
        }
    },
    template:
        '<div style="max-width: 500px; padding: 15px;">' +
        '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Name</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write name of the jedi" id="jedi_request_name" v-model="jedi_request_name">\n' +
        '</div>' +
        '<div class="text-field">\n' +
        '      <label class="text-field__label" for="username">Price</label>\n' +
        '      <input class="text-field__input" placeholder="Write price" id="jedi_request_price" v-model="jedi_request_price">\n' +
        '</div>' +
        '<input type="button" value="Send" @click="save" />' +
        '</div>',
    methods: {
        save: function() {

            this.jedi_request_name = document.getElementById("jedi_request_name").value;
            this.jedi_request_price = document.getElementById("jedi_request_price").value;

            var request = {
                name: this.jedi_request_name,
                price: parseInt(this.jedi_request_price),
            };

            JediRequestApi.save(request).then(result =>
                result.json().then(msg => {
                    this.requests.push([msg.name, msg.price]);
                    this.jedi_request_name = '';
                    this.jedi_request_price = '';
                })
            )
        }
    }
});


var myJediApi = Vue.resource('/my_jedi_data{/id}');
Vue.component('jedi-my-data-list', {
    data: function () {
        return {
            id: ''
        }
    },
    props: ['my_jedi_data'],
    template:
        '<div>' +
        '<table>' +
        '<tr><th colspan="4" width="70%">My Jedi Data</th><th></th></tr>' +
        '<tr class="timer"><td>Jedi</td><td> Planet </td><td> Team-Size </td><td> Ship-Power </td></tr>' +
        '<tr v-for="message in my_jedi_data">' +
        '<td><i>{{ message[0] }}</i></td><td> {{ message[1] }} </td><td>{{ message[2] }}</td><td>{{ message[3] }}</td>' +
        '</tr>' +
        '</table>' +
        '</div>',
    created: function () {
        this.id = 1;
        myJediApi.get({id: this.id}).then(result =>
            result.json().then(data =>
                data.forEach(msg => this.my_jedi_data.push(msg))
            )
        )
    }
});


var JediDataApi = Vue.resource('/inquisitor/data');
Vue.component('jedi-buy-form', {
    props: ['jedi_pre_data'], // , 'messageAttr'
    data: function() {
        return {
            jedi_buy_id: '',
        }
    },
    template:
        '<div>' +
        '<div style="max-width: 300px;">' +
            '<div class="text-field">\n' +
            '      <label class="text-field__label" for="username">Id</label>\n' +
            '      <input class="text-field__input" type="text" placeholder="Write id of the info you want to buy" id="jedi_buy_id" v-model="jedi_buy_id" />' +
            '</div>' +
        '</div>' +
        // '<input type="text" placeholder="Write name of the jedi" id="jedi_buy_id" v-model="jedi_buy_id" />' +
        '<input type="button" value="Save" @click="save" />' +
        '</div>',
    methods: {
        save: function() {

            this.jedi_buy_id = document.getElementById("jedi_buy_id").value;

            var message = {
                buy_id: parseInt(this.jedi_buy_id),
                jedi_id: 1
            };

            JediDataApi.save(message).then(result =>
                result.json().then(msg => {
                    // обновить страницу
                    console.log(msg)
                    this.jedi_buy_id = '';
                    location.reload();
                })
            )
        }
    }
});

Vue.component('jedi-pre-data-list', {
    props: ['jedi_pre_data'],
    template:
        '<div>' +
        '<jedi-buy-form :jedi_pre_data="jedi_pre_data"/>' +
        '<table>' +
        '<tr><th colspan="4" width="70%">Jedi Data</th><th></th></tr>' +
        '<tr class="timer"><td>ID</td><td>Jedi</td><td>Normal</td><td>Price</td></tr>' +
        '<tr v-for="message in jedi_pre_data">' +
        '<td><i>{{ message[2] }}</i></td><td><i>{{ message[0] }}</i></td><td>{{ message[1] }}</td><td>{{ message[3] }}</td>' +
        '</tr>' +
        '</table>' +
        '</div>',
    created: function () {
        JediDataApi.get().then(result =>
            result.json().then(data =>
                data.forEach(msg => this.jedi_pre_data.push(msg))
            )
        )
    }
});

var app = new Vue({
    el: '#app',
    template:
        '<div>' +
        '<div class=".overlay">' +
            '<inquisitor-money />' +
            '<jedi-request-form :requests="requests"/>' +
            '<jedi-requests-list :requests="requests" />' +
            '<jedi-my-data-list :my_jedi_data="my_jedi_data" />' +
            '<div style="margin-left: 35%; z-index: 15; margin-top: -20%">' +
                '<jedi-pre-data-list :jedi_pre_data="jedi_pre_data" />' +
                '<div style="margin-left: 50%; z-index: 16; margin-top: -37%">' +
                    '<jedi-list :jedi="jedi" />' +
                '</div>' +
            '</div>' +
        '</div>' +
        '<div>' +
        '<div class="space layer1"></div>' +
        '<div class="space layer2"></div>' +
        '<div class="space layer3"></div>' +
        '<div class="wrapper">' +
        '  <div class="sphere">' +
        '    <div class="weapon"></div>' +
        '  </div>' +
        '</div>' +
        '<div class="planet"></div>' +
        '</div>' +
        '</div>'
    ,
    data: {
        messages: [],
        requests: [],
        jedi: [],
        my_jedi_data: [],
        jedi_pre_data: []
    }
});

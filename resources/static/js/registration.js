
var messageApi = Vue.resource('/login');

Vue.component('login-form', {

    props: ['messages'],
    data: function () {
        return {
            login: '',
            password: '',
            id: ''
        }
    },

    template:
        '<form method="get" action="/login">' +
        '<div class="text-field">\n' +
        // '      <label class="text-field__label" for="username">Name</label>\n' +
        '      <input class="text-field__input" type="text" placeholder="Write Name" id="name" name="name" />' +
        '</div>' +
        '<select class="select" name="user_type">\n' +
        '    <option selected="selected" value="normal">Normal</option>\n' +
        '    <option value="inquisitor">Inquizitor</option>\n' +
        '    <option value="hunter">Hunter</option>\n' +
        '  </select>' +
        // '<p><b>Name:</b><input type="text" id="name" v-model="name" name="name" size="40"></p>' +
        '<div style="margin-top: 10px" ></div>' +
        '<input class="submit" id="subbutton" type="submit" value="Sign In" />' +
        '</form>',

    methods: {
        sbm: function () {

            let user = {
                login: document.getElementById("login").value,
                password: document.getElementById("password").value
            };

            messageApi.save({}, user).then(
                result =>
                    result.json().then(data => {
                    })
            )
        }
    },

});
var app = new Vue({
    el: '#app',
    template: '<login-form :messages="messages" />',
    data: {
        messages: []
    }
});
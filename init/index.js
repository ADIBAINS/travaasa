const mongoose = require('mongoose');
const Listing = require("../models/listing.js");
const User = require("../models/user.js");
const initdata = require('./data.js');
const MONGO_URL = "mongodb://127.0.0.1:27017/travaasa";
const mbxGeocoding = require('@mapbox/mapbox-sdk/services/geocoding');
const geoCodingClient = mbxGeocoding({ accessToken: "pk.eyJ1IjoiYWRpYmFpbnMiLCJhIjoiY21lZHl0dzVjMGd2NzJsczh5OXBoOHN4OCJ9.G6UlVxPp5hppQdGiHsvAoQ" });

main().then(() => {
    console.log("Connection Established Successfully");
}).catch((err) => {
    console.log(err);
});




async function main() {
    await mongoose.connect(MONGO_URL);
}


const initDB = async () => {
    await Listing.deleteMany({});
    const users = await User.find();
    const datalen = initdata.data.length;
    for (let countListing = 0; countListing < datalen; countListing++) {
        // Assign owner in a round-robin fashion
        const ownerId = users[countListing % users.length]._id.toString();
        initdata.data[countListing].owner = ownerId;
        let response = await geoCodingClient.forwardGeocode({
            query: initdata.data[countListing].location,
            limit: 1
        }).send();
        initdata.data[countListing].geometry = response.body.features[0].geometry;
        let datatoin = initdata.data[countListing];
        await Listing.insertOne(datatoin);
    }
    console.log("Data was Initialised");
}

initDB();
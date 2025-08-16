const Listing = require('../models/listing');
const mbxGeocoding = require('@mapbox/mapbox-sdk/services/geocoding');
const MAPTOKEN = JSON.stringify(process.env.MAPBOX_TOKEN);
const geoCodingClient = mbxGeocoding({ accessToken: MAPTOKEN });

module.exports.index = async (req, res) => {
    const allListings = await Listing.find({});
    res.render("listings/index.ejs", { allListings, MAPTOKEN }); // <-- pass mapToken here
};


module.exports.showListing = async (req, res) => {
    let { id } = req.params;
    const reqListings = await Listing.findById(id)
        .populate({ path: 'review', populate: { path: "author" } })
        .populate("owner");
    if (!reqListings) {
        req.flash("error", "No such Listing Found!");
        return res.redirect("/listings");
    }
    res.render('listings/show.ejs', { reqListings, MAPTOKEN }); // <-- pass mapToken here
};

module.exports.renderNewForm = (req, res) => {
    res.render("listings/new.ejs");
};

module.exports.uploadNewListing = async (req, res) => {
    let response = await geoCodingClient.forwardGeocode({
  query: req.body.listing.location,
  limit: 1
})
  .send();
  
    let url = req.file.path;
    let filename = req.file.filename;
    const data = req.body.listing;
    const newListing = new Listing(data);
    newListing.owner = req.user._id;
    newListing.image = { url, filename };
    newListing.geometry =response.body.features[0].geometry;
    await newListing.save();
    req.flash("success", "New Listing Created!");
    res.redirect("/listings");
};

module.exports.renderEditForm = async (req, res) => {
    const { id } = req.params;
    const data = await Listing.findById(id);
    if (!data) {
        req.flash("error", "No such Listing Found!");
        return res.redirect("/listings");
    }
    res.render("listings/edit.ejs", { data });
};

module.exports.uploadEditData = async (req, res) => {
    const { id } = req.params;
    const data = req.body;
    let listing = await Listing.findByIdAndUpdate(id, { ...data.listing });
    if (typeof req.file !== "undefined") {
        let url = req.file.path;
        let filename = req.file.filename;
        listing.image = { url, filename };
        await listing.save();
    }
    req.flash("success", "Listing Updated Successfully!");
    res.redirect(`/listings/${id}`);

};

module.exports.destroyListing = async (req, res) => {
    const { id } = req.params;
    let listing = await Listing.findByIdAndDelete(id);
    if (!listing) {
        req.flash("error", "No such Listing Found!");
        return res.redirect("/listings");
    }
    req.flash("success", "Listing Deleted Successfully!");
    res.redirect("/listings");
};
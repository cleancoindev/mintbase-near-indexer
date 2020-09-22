table! {
		use diesel::sql_types::*;
	stores (
		id,
		owner,
		name,
		symbol,
	) {
		id -> Text,
		owner -> Text,
		name -> Text,
		symbol -> Text,
		totalSupply -> BigInt,
		timestamp -> Text,
		burned -> Bool,
		tokenCount -> Numeric,
		boughtCount -> Numeric,
		valueCount -> Numeric,
		transferCount -> Numeric,
		}
}

table! {
things (
	id,
	minter,
	timestamp,
	burned,
	forSale,
	metaId

) {
	id -> Text,
	minter -> Text,
	timestamp -> Text,
	symbol -> Text,
	metaId -> Text,
	forSale -> Bool,
	burned -> Bool,
	}
}

table! {
tokens (
	id,
	tokenId,
	metaId,
	price,
	burend,
	state,
	transferCount,
) {
	id -> Text,
	tokenId -> Text,
	metaId -> Text,
	price -> Text,
	burend -> Bool,
	state -> Text,
	transferCount -> BigInt,
	}
}

table! {
users (
	id,
	avitar,
) {
	id -> Text,
	avitar -> Text,
	}
}

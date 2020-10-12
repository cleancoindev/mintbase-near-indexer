table! {
		use diesel::sql_types::*;
	stores (
		id,
		owner,
		name,
		symbol,
		totalSupply,
		burned,
		tokenCount,
		boughtCount,
		valueCount,
		transferCount
	) {
		id -> Text,
		owner -> Text,
		name -> Text,
		symbol -> Text,
		totalSupply -> BigInt,
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
	burned,
	forSale,
	metaId,
	resolveStore
) {
	id -> Text,
	minter -> Text,
	burned -> Bool,
	forSale -> Bool,
	metaId -> Text,
	resolveStore -> Text,
	}
}

table! {
tokens (
	id,
	tokenId,
	metaId,
	price,
	burned,
	state,
	transferCount,
	storeId,
	ownerId
) {
	id -> Text,
	tokenId -> Text,
	metaId -> Text,
	price -> Text,
	burned -> Bool,
	state -> Text,
	transferCount -> Numeric,
	storeId -> Text,
	ownerId -> Text,
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

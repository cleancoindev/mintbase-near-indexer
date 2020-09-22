curl -X POST -H "Content-Type: application/json" \
 -d '{"marketId": 0, "date": "2020-07-11" }' \
 localhost:3001/market/get_avg_prices_for_date

#!/bin/bash
set -e

echo "🔍 Running RabitHole production smoke tests..."

BASE_URL="${RABITHOLE_URL:-http://127.0.0.1:3000}"

# Test 1: Health check
echo "1. Testing health endpoint..."
RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/health")
if [ "$RESPONSE" = "200" ]; then
    echo "   ✅ Health check passed"
else
    echo "   ❌ Health check failed: HTTP $RESPONSE"
    exit 1
fi

# Test 2: Search endpoint
echo "2. Testing search endpoint..."
RESPONSE=$(curl -s "$BASE_URL/search?q=physics")
if echo "$RESPONSE" | jq -e '.query == "physics"' > /dev/null 2>&1; then
    echo "   ✅ Search endpoint passed"
else
    echo "   ❌ Search endpoint failed"
    exit 1
fi

# Test 3: Topics endpoint
echo "3. Testing topics endpoint..."
RESPONSE=$(curl -s "$BASE_URL/topics/test-topic")
if echo "$RESPONSE" | jq -e '.topic.slug == "test-topic"' > /dev/null 2>&1; then
    echo "   ✅ Topics endpoint passed"
else
    echo "   ❌ Topics endpoint failed"
    exit 1
fi

# Test 4: Daily trail endpoint
echo "4. Testing daily trail endpoint..."
RESPONSE=$(curl -s "$BASE_URL/api/daily-trail")
if echo "$RESPONSE" | jq -e '.topic_ids != null' > /dev/null 2>&1; then
    echo "   ✅ Daily trail endpoint passed"
else
    echo "   ❌ Daily trail endpoint failed"
    exit 1
fi

# Test 5: Streak endpoint
echo "5. Testing streak endpoint..."
RESPONSE=$(curl -s "$BASE_URL/api/streak")
if echo "$RESPONSE" | jq -e '. != null' > /dev/null 2>&1; then
    echo "   ✅ Streak endpoint passed"
else
    echo "   ❌ Streak endpoint failed"
    exit 1
fi

# Test 6: Waitlist endpoint
echo "6. Testing waitlist endpoint..."
RESPONSE=$(curl -s -X POST "$BASE_URL/api/waitlist" -H "Content-Type: application/json" -d '{"email":"test@example.com"}')
if echo "$RESPONSE" | jq -e '.success == true' > /dev/null 2>&1; then
    echo "   ✅ Waitlist endpoint passed"
else
    echo "   ❌ Waitlist endpoint failed"
    exit 1
fi

echo ""
echo "🎉 All smoke tests passed!"

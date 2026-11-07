import { useState, useEffect } from 'react'
import axios from 'axios'

// Types for the graph
type TopicData = {
  id: string
  title: string
  slug: string
  summary: string
  quizQuestions?: QuizQuestion[]
}

type RelatedTopic = {
  id: number
  title: string
  slug: string
  relationType: string
}

type MapNode = {
  id: string
  title: string
  slug: string
  summary: string
  x: number
  y: number
}

type MapEdge = {
  source: string
  target: string
}

type SavedMap = {
  id: string
  title: string
  nodes: MapNode[]
  edges: MapEdge[]
  createdAt: number
}

type QuizQuestion = {
  id: number
  question: string
  options: string[]
  correctOption: number
}

type QuizState = {
  active: boolean
  questionIndex: number
  score: number
  answers: number[]
}

const MAX_MAPS = 5

export default function App() {
  const [searchQuery, setSearchQuery] = useState('')
  const [searchResults, setSearchResults] = useState<TopicData[]>([])
  const [selectedTopic, setSelectedTopic] = useState<TopicData | null>(null)
  const [graphNodes, setGraphNodes] = useState<MapNode[]>([])
  const [graphEdges, setGraphEdges] = useState<MapEdge[]>([])
  const [savedMaps, setSavedMaps] = useState<SavedMap[]>([])
  const [currentMapId, setCurrentMapId] = useState<string | null>(null)
  const [quizState, setQuizState] = useState<QuizState>({
    active: false,
    questionIndex: 0,
    score: 0,
    answers: [],
  })

  // Load saved maps from localStorage on mount
  useEffect(() => {
    const saved = localStorage.getItem('rabithole_maps')
    if (saved) {
      setSavedMaps(JSON.parse(saved))
    }
  }, [])

  // Save maps to localStorage whenever they change
  useEffect(() => {
    localStorage.setItem('rabithole_maps', JSON.stringify(savedMaps))
  }, [savedMaps])

  // Search for topics
  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!searchQuery.trim()) return

    setSearchResults([])
    try {
      const response = await axios.get(`/api/search?q=${encodeURIComponent(searchQuery)}`)
      setSearchResults(response.data.results)
    } catch (err) {
      console.error('Search error:', err)
    }
  }

  // Load a topic and its related topics to build the graph
  const loadTopic = async (topic: TopicData) => {
    setSelectedTopic({ ...topic, quizQuestions: [] })
    setSearchResults([])
    setGraphNodes([])
    setGraphEdges([])

    try {
      const response = await axios.get(`/api/topics/${topic.slug}`)
      const data = response.data

      // Build initial graph: topic + related topics
      const nodes: MapNode[] = [
        {
          id: data.topic.slug,
          title: data.topic.title,
          slug: data.topic.slug,
          summary: data.topic.summary || '',
          x: 0,
          y: 0,
        },
      ]

      const edges: MapEdge[] = []

      // Add up to 10 related topics (depth 1)
      const maxRelated = Math.min(data.related.length, 10)
      for (let i = 0; i < maxRelated; i++) {
        const related = data.related[i]
        nodes.push({
          id: related.slug,
          title: related.title,
          slug: related.slug,
          summary: '',
          x: 0,
          y: 0,
        })
        edges.push({ source: data.topic.slug, target: related.slug })
      }

      // Position nodes in a simple layout
      const positionedNodes = positionNodes(nodes)

      setGraphNodes(positionedNodes)
      setGraphEdges(edges)

      // Update selected topic with quiz questions from API
      setSelectedTopic({
        ...data.topic,
        quizQuestions: data.topic.quizQuestions || [],
      })
    } catch (err) {
      console.error('Load topic error:', err)
    }
  }

  // Position nodes around the center
  const positionNodes = (nodes: MapNode[]): MapNode[] => {
    const result = [...nodes]
    if (result.length <= 1) return result

    result[0].x = 0
    result[0].y = 0

    for (let i = 1; i < result.length; i++) {
      const angle = (i / result.length) * Math.PI * 2
      const radius = 200 * Math.sqrt(i)
      result[i].x = Math.cos(angle) * radius
      result[i].y = Math.sin(angle) * radius
    }

    return result
  }

  // Add a node to the current map (when user expands a node)
  const addNodeToMap = (node: MapNode) => {
    // Check if node already exists in the map
    if (graphNodes.some((n) => n.id === node.id)) return

    const newNodes = [...graphNodes, node]
    setGraphNodes(newNodes)

    // Connect to the previously selected node
    if (selectedTopic) {
      setGraphEdges([...graphEdges, { source: selectedTopic.slug, target: node.id }])
    }
  }

  // Reset quiz state
  const resetQuiz = () => setQuizState({
    active: false,
    questionIndex: 0,
    score: 0,
    answers: [],
  })

  // Submit quiz answer and move to next question
  const answerQuiz = (optionIndex: number) => {
    const q = selectedTopic?.quizQuestions?.[quizState.questionIndex]
    if (!q) return

    const isCorrect = optionIndex === q.correctOption
    const newScore = quizState.score + (isCorrect ? 1 : 0)
    const newAnswers = [...quizState.answers, optionIndex]

    if (quizState.questionIndex < (selectedTopic?.quizQuestions?.length || 0) - 1) {
      setQuizState({
        active: true,
        questionIndex: quizState.questionIndex + 1,
        score: newScore,
        answers: newAnswers,
      })
    } else {
      setQuizState({
        active: false,
        questionIndex: selectedTopic?.quizQuestions?.length || 0,
        score: newScore,
        answers: newAnswers,
      })
    }
  }

  // Search for topics
  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!searchQuery.trim()) return

    setSearchResults([])
    try {
      const response = await axios.get(`/api/search?q=${encodeURIComponent(searchQuery)}`)
      setSearchResults(response.data.results)
    } catch (err) {
      console.error('Search error:', err)
    }
  }

  // Load a topic and its related topics to build the graph
  const loadTopic = async (topic: TopicData) => {
    setSelectedTopic({ ...topic, quizQuestions: [] })
    setSearchResults([])
    setGraphNodes([])
    setGraphEdges([])

    try {
      const response = await axios.get(`/api/topics/${topic.slug}`)
      const data = response.data

      // Build initial graph: topic + related topics
      const nodes: MapNode[] = [
        {
          id: data.topic.slug,
          title: data.topic.title,
          slug: data.topic.slug,
          summary: data.topic.summary || '',
          x: 0,
          y: 0,
        },
      ]

      const edges: MapEdge[] = []

      // Add up to 10 related topics (depth 1)
      const maxRelated = Math.min(data.related.length, 10)
      for (let i = 0; i < maxRelated; i++) {
        const related = data.related[i]
        nodes.push({
          id: related.slug,
          title: related.title,
          slug: related.slug,
          summary: '',
          x: 0,
          y: 0,
        })
        edges.push({ source: data.topic.slug, target: related.slug })
      }

      // Position nodes in a simple layout
      const positionedNodes = positionNodes(nodes)

      setGraphNodes(positionedNodes)
      setGraphEdges(edges)

      // Update selected topic with quiz questions from API
      setSelectedTopic({
        ...data.topic,
        quizQuestions: data.topic.quizQuestions || [],
      })
    } catch (err) {
      console.error('Load topic error:', err)
    }
  }

  // Position nodes around the center
  const positionNodes = (nodes: MapNode[]): MapNode[] => {
    const result = [...nodes]
    if (result.length <= 1) return result

    result[0].x = 0
    result[0].y = 0

    for (let i = 1; i < result.length; i++) {
      const angle = (i / result.length) * Math.PI * 2
      const radius = 200 * Math.sqrt(i)
      result[i].x = Math.cos(angle) * radius
      result[i].y = Math.sin(angle) * radius
    }

    return result
  }

  // Add a node to the current map (when user expands a node)
  const addNodeToMap = (node: MapNode) => {
    // Check if node already exists in the map
    if (graphNodes.some((n) => n.id === node.id)) return

    const newNodes = [...graphNodes, node]
    setGraphNodes(newNodes)

    // Connect to the previously selected node
    if (selectedTopic) {
      setGraphEdges([...graphEdges, { source: selectedTopic.slug, target: node.id }])
    }
  }

  // Delete a saved map
  const deleteMap = (mapId: string) => {
    setSavedMaps((prev) => prev.filter((m) => m.id !== mapId))
  }

  // Load a saved map
  const loadMap = (mapId: string) => {
    const map = savedMaps.find((m) => m.id === mapId)
    if (map) {
      setGraphNodes(map.nodes)
      setGraphEdges(map.edges)
      setCurrentMapId(mapId)
    }
  }

  // Save the current map
  const saveMap = () => {
    if (!selectedTopic) return

    const mapTitle = selectedTopic.title || 'Untitled Map'
    const newMap: SavedMap = {
      id: Date.now().toString(),
      title: mapTitle,
      nodes: graphNodes,
      edges: graphEdges,
      createdAt: Date.now(),
    }

    // Enforce 5-map limit
    if (savedMaps.length >= MAX_MAPS) {
      // Remove the oldest map
      const sorted = [...savedMaps].sort((a, b) => a.createdAt - b.createdAt)
      const toRemove = sorted[0].id
      const remaining = sorted.slice(1).map((m) => m.id)
      setSavedMaps((prev) => prev.filter((m) => !remaining.includes(m.id) && m.id !== toRemove))
    }

    setSavedMaps((prev) => [...prev, newMap])
  }

  // Load a saved map
  const loadMap = (mapId: string) => {
    const map = savedMaps.find((m) => m.id === mapId)
    if (map) {
      setGraphNodes(map.nodes)
      setGraphEdges(map.edges)
      setCurrentMapId(mapId)
    }
  }

  // Delete a saved map
  const deleteMap = (mapId: string) => {
    setSavedMaps((prev) => prev.filter((m) => m.id !== mapId))
  }

  return (
    <div style={{ minHeight: '100vh', padding: '20px' }}>
      <div style={{ marginBottom: '20px' }}>
        <h2>RabitHole - Phase 2</h2>
        <form onSubmit={handleSearch} style={{ display: 'flex', gap: '10px', marginBottom: '16px' }}>
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search a topic..."
            style={{ flex: 1, padding: '8px', border: '1px solid #e5e4e7', borderRadius: '4px' }}
          />
          <button type="submit" style={{ padding: '8px 16px', background: '#aa3bff', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}>
            Explore
          </button>
        </form>

        {searchResults.length > 0 && (
          <div>
            <h3>Search results for "{searchQuery}"</h3>
            <ul>
              {searchResults.map((result) => (
                <li
                  key={result.slug}
                  onClick={() => loadTopic(result)}
                  style={{ cursor: 'pointer', margin: '4px 0', padding: '8px', border: '1px solid #e5e4e7', borderRadius: '4px' }}
                >
                  {result.title}
                </li>
              ))}
            </ul>
          </div>
        )}

        {selectedTopic && (
          <div style={{ marginBottom: '24px', padding: '16px', border: '1px solid #e5e4e7', borderRadius: '8px' }}>
            <h3>{selectedTopic.title}</h3>
            <p>{selectedTopic.summary || 'No summary available'}</p>

            {selectedTopic.quizQuestions && selectedTopic.quizQuestions.length > 0 && (
              <div style={{ marginTop: '12px' }}>
                <h4>Quiz</h4>
                <p>{selectedTopic.quizQuestions[quizState.questionIndex].question}</p>
                {selectedTopic.quizQuestions[quizState.questionIndex].options.map((opt, i) => (
                  <button
                    key={i}
                    onClick={() => answerQuiz(i)}
                    style={{
                      width: '100%',
                      margin: '4px 0',
                      padding: '8px',
                      background: quizState.active && quizState.answers[quizState.questionIndex] === i ? '#e5e4e7' : 'transparent',
                      border: '1px solid #e5e4e7',
                      borderRadius: '4px',
                      color: quizState.active && quizState.answers[quizState.questionIndex] === i ? '#fff' : '#333',
                      cursor: !quizState.active ? 'default' : 'pointer',
                      fontWeight: quizState.active && quizState.answers[quizState.questionIndex] === i ? 'bold' : 'normal',
                    }}
                    disabled={quizState.active}
                  >
                    {opt}
                  </button>
                ))}
                {quizState.active && quizState.questionIndex < selectedTopic.quizQuestions.length - 1 && (
                  <p style={{ marginTop: '8px', fontSize: '12px', color: '#6b6375' }}>
                    Answer {quizState.questionIndex + 1} of {selectedTopic.quizQuestions.length}
                  </p>
                )}
                {!quizState.active && selectedTopic.quizQuestions.length > 0 && (
                  <p style={{ marginTop: '8px', textAlign: 'right' }}>
                    Score: {quizState.score} / {selectedTopic.quizQuestions.length}
                  </p>
                )}
              </div>
            )}

            <button
              onClick={() => loadTopic(selectedTopic)}
              style={{
                marginTop: '8px',
                padding: '6px 12px',
                background: '#f0f0f0',
                border: '1px solid #e5e4e7',
                borderRadius: '4px',
                cursor: 'pointer',
              }}
            >
              Load topic (reload graph)
            </button>
          </div>
        )}

        {/* Graph visualization area */}
        <div
          id="graph-area"
          style={{
            minHeight: '400px',
            minWidth: '600px',
            border: '2px solid #e5e4e7',
            borderRadius: '8px',
            position: 'relative',
            margin: '20px 0',
          }}
        >
          {graphNodes.length > 0 && graphEdges.length > 0 && (
            <div id="d3-graph" style={{ width: '100%', height: '100%' }}>
              {/* D3 will be initialized in useEffect below */}
            </div>
          )}
          {graphNodes.length === 0 && <p style={{ textAlign: 'center', padding: '20px' }}>Search for a topic or select one to explore the graph.</p>}
        </div>

        {/* Node expansion panel (placeholder for click-to-expand) */}
        {selectedTopic && graphNodes.some((n) => n.id !== selectedTopic.slug) && (
          <div style={{ marginTop: '16px' }}>
            <h4>Expand topic</h4>
            <p>Click on any node in the graph to expand it and see its related topics.</p>
          </div>
        )}

        {/* Saved maps sidebar */}
        <div style={{ marginTop: '24px', borderTop: '1px solid #e5e4e7', paddingTop: '20px' }}>
          <h3>Saved Maps ({savedMaps.length}/{MAX_MAPS})</h3>
          {savedMaps.length === 0 ? (
            <p style={{ color: '#6b6375' }}>No maps saved yet. Explore topics and save maps.</p>
          ) : (
            <ul>
              {savedMaps.map((map) => (
                <li
                  key={map.id}
                  style={{
                    listStyle: 'none',
                    margin: '8px 0',
                    padding: '8px',
                    border: '1px solid #e5e4e7',
                    borderRadius: '4px',
                    cursor: 'pointer',
                  }}
                  onClick={() => loadMap(map.id)}
                >
                  <strong>{map.title}</strong> ({map.nodes.length} nodes)
                  <button
                    style={{
                      float: 'right',
                      marginLeft: '8px',
                      padding: '4px 8px',
                      background: 'transparent',
                      border: 'none',
                      color: '#e5e4e7',
                      cursor: 'pointer',
                    }}
                    onClick={(e) => {
                      e.stopPropagation()
                      deleteMap(map.id)
                    }}
                  >
                    Delete
                  </button>
                </li>
              ))}
            </ul>
          )}
          <button
            onClick={() => saveMap()}
            style={{
              marginTop: '8px',
              width: '100%',
              padding: '8px',
              background: '#aa3bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
            }}
          >
            Save current map
          </button>
        </div>
      </div>
    </div>
  )
}
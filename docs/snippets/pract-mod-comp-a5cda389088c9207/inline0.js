

// Función para calcular los puntos de la recta

function calcularRecta(W, limites) {
    const [xMin, xMax] = limites;
    const xVals = [xMin, xMax];
    const yVals = xVals.map(x => (-W[0] * x + W[2]) / W[1]);
    return xVals.map((x, i) => [x, yVals[i]]);
}

function calcularLimites(allPoints) {

    // Inicializar los límites con valores extremos
    let minX = Infinity;
    let maxX = -Infinity;

    // Recorrer todos los puntos para encontrar los límites
    allPoints.forEach(point => {
        if (point[0] < minX) {
            minX = point[0];
        }
        if (point[0] > maxX) {
            maxX = point[0];
        }
    });

    // Añadir un margen de 0.5 a los límites
    minX -= 0.5;
    maxX += 0.5;

    // Redondear los límites al múltiplo más cercano de 0.5
    minX = (Math.round(minX * 2) / 2).toFixed(1);
    maxX = (Math.round(maxX * 2) / 2).toFixed(1);

    return [minX, maxX];
}

export function ps_chart(dataPoints, weights) {

    const limites = calcularLimites(dataPoints);

    
    // Filtrar los puntos en función de su tipo
    const positivePoints = dataPoints.filter(point => point[2] === 1).map(point => [point[0], point[1]]);
    const negativePoints = dataPoints.filter(point => point[2] === -1).map(point => [point[0], point[1]]);


    // Cargar el tema
    fetch('public/themes/roma.json')
        .then(response => response.json())
        .then(theme => {
            echarts.registerTheme('custom_theme', theme);

            // Configuración de la gráfica en ECharts
            const option = {
                color: theme.color, // Establecer la paleta de colores global
                timeline: {
                    axisType: 'category',
                    autoPlay: false,
                    playInterval: 1500,
                    data: weights.map(() => ''),
                    tooltip: {
                        formatter: function(params) {
                            return `Configuración de pesos ${params.dataIndex + 1}`;
                        }
                    },
                    lineStyle: theme.timeline.lineStyle,
                    itemStyle: theme.timeline.itemStyle,
                    progress: {
                        lineStyle: theme.timeline.lineStyle,
                        itemStyle: theme.timeline.itemStyle
                    },
                    controlStyle: theme.timeline.controlStyle,
                    checkpointStyle: theme.timeline.checkpointStyle,
                    label: theme.timeline.label
                },
                options: weights.map((W, index) => ({
                    title: {
                        left: 'center'
                    },
                    xAxis: {
                        type: 'value',
                        min: limites[0],
                        max: limites[1]
                    },
                    yAxis: {
                        type: 'value',
                        min: limites[0],
                        max: limites[1]
                    },
                    series: [
                        {
                            type: 'scatter',
                            data: positivePoints,
                            symbolSize: 12,
                            name: 'Tipo 1',
                            itemStyle: {
                                color: theme.color[12]
                            }
                        },
                        {
                            type: 'scatter',
                            data: negativePoints,
                            symbolSize: 12,
                            name: 'Tipo -1',
                            itemStyle: {
                                color: theme.color[11]
                            }
                        },
                        {
                            type: 'line',
                            data: calcularRecta(W, limites),
                            name: 'Recta',
                            itemStyle: {
                                color: theme.color[5] // Color para los puntos de la línea
                            },
                            lineStyle: theme.line.lineStyle,
                            emphasis: {
                                lineStyle: {
                                    color: theme.color[5] // Mantener el color al hacer hover
                                },
                                itemStyle: {
                                    color: theme.color[5] // Mantener el color al hacer hover
                                }
                            }
                        }
                    ]
                }))
            };

            const chart = echarts.init(document.getElementById('ps-chart'), 'custom_theme');
            window.addEventListener('resize', function() {
                myChart.resize();
            });
            chart.setOption(option);
        });
}

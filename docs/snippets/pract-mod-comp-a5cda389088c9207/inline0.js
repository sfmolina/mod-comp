

// Función para calcular los puntos de la recta
function calcularRecta(W, limites) {
    const [xMin, xMax] = limites;
    const xVals = [xMin, xMax];
    const yVals = xVals.map(x => (-W[0] * x + W[2]) / W[1]);
    return xVals.map((x, i) => [x, yVals[i]]);
}

export function ps_chart(dataPoints, weights) {
    const limites = [-1.0, 2.5]; // Limites del eje X

    // Filtrar los puntos en función de su tipo
    const positivePoints = dataPoints.filter(point => point[2] === 1).map(point => [point[0], point[1]]);
    const negativePoints = dataPoints.filter(point => point[2] === -1).map(point => [point[0], point[1]]);

    // Configuración de la gráfica en ECharts
    const option = {
        timeline: {
            axisType: 'category',
            autoPlay: false,
            playInterval: 1500,
            data: weights.map(() => ''), // Genera entradas vacías para cada conjunto de pesos
            tooltip: {
                formatter: function(params) {
                    return `Configuración de pesos ${params.dataIndex + 1}`;
                }
            }
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
                    data: positivePoints, // Puntos de tipo 1
                    symbolSize: 10,
                    name: 'Tipo 1',
                    itemStyle: {
                        color: '#4820b6' // Color para el tipo 1
                    }
                },
                {
                    type: 'scatter',
                    data: negativePoints, // Puntos de tipo -1
                    symbolSize: 10,
                    name: 'Tipo -1',
                    itemStyle: {
                        color: '#dd4340' // Color para el tipo -1
                    }
                },
                {
                    type: 'line',
                    data: calcularRecta(W, limites), // La recta cambia en cada paso
                    lineStyle: {
                        width: 2
                    },
                    name: 'Recta'
                }
            ]
        }))
    };

    // Inicializar el gráfico
    const chart = echarts.init(document.getElementById('ps-chart'));
    chart.setOption(option);
}
